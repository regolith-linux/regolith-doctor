//! The result of one run and how it maps to an exit status.

use serde::{Deserialize, Serialize};

use crate::finding::{Finding, Severity};
use crate::id::CheckId;

/// Version of the JSON report format. Bump it on any incompatible change.
pub const SCHEMA_VERSION: u32 = 1;

/// A check that could not run to completion (for example an unreadable
/// file). It is not a finding about the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckFailure {
    /// The check that failed.
    pub id: CheckId,
    /// What went wrong.
    pub message: String,
}

/// Everything one `check` run produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    /// Always [`SCHEMA_VERSION`] when produced by this version.
    pub schema_version: u32,
    /// The checks that ran, sorted.
    pub checks: Vec<CheckId>,
    /// Findings, in a deterministic order (see [`Report::finish`]).
    pub findings: Vec<Finding>,
    /// Checks that failed to run.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<CheckFailure>,
}

impl Default for Report {
    fn default() -> Self {
        Report {
            schema_version: SCHEMA_VERSION,
            checks: Vec::new(),
            findings: Vec::new(),
            failures: Vec::new(),
        }
    }
}

impl Report {
    /// Sorts everything so that the same system always gives the same
    /// output: checks by id; findings by id, then most severe first, then by
    /// the first evidence path and line, then by message.
    pub fn finish(&mut self) {
        self.checks.sort_unstable();
        self.checks.dedup();
        self.failures.sort_by_key(|f| f.id);
        self.findings.sort_by(|a, b| {
            let first = |f: &Finding| {
                f.evidence
                    .first()
                    .map(|e| (e.path.clone(), e.line))
                    .unwrap_or_default()
            };
            a.id.cmp(&b.id)
                .then(b.severity.cmp(&a.severity))
                .then_with(|| first(a).cmp(&first(b)))
                .then_with(|| a.message.cmp(&b.message))
        });
    }

    /// The most severe finding, if any.
    pub fn max_severity(&self) -> Option<Severity> {
        self.findings.iter().map(|f| f.severity).max()
    }

    /// Count of findings with the given severity.
    pub fn count(&self, severity: Severity) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .count()
    }

    /// The exit status this report maps to.
    pub fn exit_status(&self) -> ExitStatus {
        if !self.failures.is_empty() {
            return ExitStatus::Failure;
        }
        match self.max_severity() {
            Some(Severity::Error) => ExitStatus::Errors,
            Some(Severity::Warning) => ExitStatus::Warnings,
            Some(Severity::Info) | None => ExitStatus::Clean,
        }
    }
}

/// Process exit statuses of `regolith-doctor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitStatus {
    /// 0: no finding at warning level or above.
    Clean,
    /// 1: at least one warning, no errors.
    Warnings,
    /// 2: at least one error.
    Errors,
    /// 3: usage error, internal error, or a check that failed to run.
    Failure,
    /// 4: the requested command or option is not implemented yet.
    NotImplemented,
}

impl ExitStatus {
    /// The numeric process exit code.
    pub const fn code(self) -> u8 {
        match self {
            ExitStatus::Clean => 0,
            ExitStatus::Warnings => 1,
            ExitStatus::Errors => 2,
            ExitStatus::Failure => 3,
            ExitStatus::NotImplemented => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finding::Evidence;
    use crate::id::Area;

    fn finding(n: u16, severity: Severity, path: &str) -> Finding {
        Finding::new(CheckId::new(Area::Env, n), severity, "m").with_evidence(Evidence::file(path))
    }

    #[test]
    fn exit_status_follows_the_worst_finding() {
        let mut report = Report::default();
        assert_eq!(report.exit_status(), ExitStatus::Clean);
        report.findings.push(finding(1, Severity::Info, "/a"));
        assert_eq!(report.exit_status(), ExitStatus::Clean);
        report.findings.push(finding(1, Severity::Warning, "/a"));
        assert_eq!(report.exit_status(), ExitStatus::Warnings);
        report.findings.push(finding(1, Severity::Error, "/a"));
        assert_eq!(report.exit_status(), ExitStatus::Errors);
        report.failures.push(CheckFailure {
            id: CheckId::new(Area::Env, 1),
            message: "boom".into(),
        });
        assert_eq!(report.exit_status(), ExitStatus::Failure);
    }

    #[test]
    fn finish_orders_findings_deterministically() {
        let mut report = Report {
            findings: vec![
                finding(2, Severity::Info, "/b"),
                finding(1, Severity::Warning, "/z"),
                finding(1, Severity::Error, "/z"),
                finding(1, Severity::Warning, "/a"),
            ],
            ..Report::default()
        };
        report.finish();
        let order: Vec<(String, Severity, Option<String>)> = report
            .findings
            .iter()
            .map(|f| (f.id.to_string(), f.severity, f.evidence[0].path.clone()))
            .collect();
        assert_eq!(
            order,
            vec![
                ("ENV-001".into(), Severity::Error, Some("/z".into())),
                ("ENV-001".into(), Severity::Warning, Some("/a".into())),
                ("ENV-001".into(), Severity::Warning, Some("/z".into())),
                ("ENV-002".into(), Severity::Info, Some("/b".into())),
            ]
        );
    }
}

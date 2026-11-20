//! What a check reports: findings with a severity, the sessions they affect,
//! evidence and fix proposals.

use serde::{Deserialize, Serialize};

use crate::id::CheckId;

/// How bad a finding is. Ordered from least to most severe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Worth knowing; nothing is broken.
    Info,
    /// Something probably does not work as the user expects.
    Warning,
    /// Something is broken or cannot work.
    Error,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
        })
    }
}

/// A desktop session a check or finding applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Session {
    /// The X11 session (i3 with gnome-flashback).
    I3,
    /// The Wayland session (sway).
    Sway,
    /// A plain GNOME session on the same machine.
    Gnome,
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Session::I3 => "i3",
            Session::Sway => "sway",
            Session::Gnome => "gnome",
        })
    }
}

/// How far the doctor may go when fixing a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FixClass {
    /// The doctor may apply the fix itself (reversible, with backup and undo).
    Auto,
    /// The doctor shows the exact change and applies it only after the user
    /// confirms.
    Guided,
    /// The doctor explains; it changes nothing.
    ReportOnly,
}

impl std::fmt::Display for FixClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FixClass::Auto => "auto",
            FixClass::Guided => "guided",
            FixClass::ReportOnly => "report-only",
        })
    }
}

/// Where a finding comes from. Paths are absolute paths as seen inside the
/// root (`/etc/os-release`), never host paths.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    /// File the finding is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 1-based line in `path`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    /// Package that owns `path`, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    /// Command or IPC probe that produced the fact, when it did not come
    /// from a file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe: Option<String>,
    /// Short extra detail, such as the offending value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Evidence {
    /// Evidence pointing at a file.
    pub fn file(path: impl Into<String>) -> Self {
        Evidence {
            path: Some(path.into()),
            ..Evidence::default()
        }
    }

    /// Adds a line number.
    pub fn at_line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }

    /// Adds a note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// A fix the doctor proposes. For now this is a description; the fix engine
/// will add concrete file operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixProposal {
    /// Safety class.
    pub class: FixClass,
    /// What the fix does, in one sentence.
    pub summary: String,
}

/// One problem found by one check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// The check that produced it.
    pub id: CheckId,
    /// How bad it is.
    pub severity: Severity,
    /// Sessions it affects, sorted.
    pub sessions: Vec<Session>,
    /// One-sentence description for people.
    pub message: String,
    /// Files, lines and probes that support it.
    #[serde(default)]
    pub evidence: Vec<Evidence>,
    /// Proposed fixes.
    #[serde(default)]
    pub fixes: Vec<FixProposal>,
}

impl Finding {
    /// A finding with no sessions, evidence or fixes yet.
    pub fn new(id: CheckId, severity: Severity, message: impl Into<String>) -> Self {
        Finding {
            id,
            severity,
            sessions: Vec::new(),
            message: message.into(),
            evidence: Vec::new(),
            fixes: Vec::new(),
        }
    }

    /// Sets the affected sessions (sorted, without duplicates).
    pub fn with_sessions(mut self, sessions: &[Session]) -> Self {
        self.sessions = sessions.to_vec();
        self.sessions.sort_unstable();
        self.sessions.dedup();
        self
    }

    /// Adds one piece of evidence.
    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    /// Adds one fix proposal.
    pub fn with_fix(mut self, class: FixClass, summary: impl Into<String>) -> Self {
        self.fixes.push(FixProposal {
            class,
            summary: summary.into(),
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::Area;

    #[test]
    fn severity_orders_by_badness() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
    }

    #[test]
    fn finding_serializes_with_stable_names() {
        let finding = Finding::new(CheckId::new(Area::Env, 1), Severity::Warning, "msg")
            .with_sessions(&[Session::Sway, Session::I3, Session::Sway])
            .with_evidence(Evidence::file("/proc/version").at_line(1))
            .with_fix(FixClass::ReportOnly, "explain");
        let json = serde_json::to_value(&finding).unwrap();
        assert_eq!(json["id"], "ENV-001");
        assert_eq!(json["severity"], "warning");
        assert_eq!(json["sessions"], serde_json::json!(["i3", "sway"]));
        assert_eq!(json["evidence"][0]["path"], "/proc/version");
        assert!(json["evidence"][0].get("package").is_none());
        assert_eq!(json["fixes"][0]["class"], "report-only");
    }
}

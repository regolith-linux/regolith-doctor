//! The [`Check`] trait and the [`Registry`] that runs checks.

use std::path::PathBuf;

use crate::finding::{Finding, FixClass, Session};
use crate::id::{Area, CheckId};
use crate::report::{CheckFailure, Report};
use crate::rootfs::{FsError, RootFs};

/// Static description of a check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckInfo {
    /// Stable identifier.
    pub id: CheckId,
    /// One-line title, the same as in `docs/checks/<ID>.md`.
    pub title: &'static str,
    /// Sessions the check is about.
    pub sessions: &'static [Session],
    /// The most the doctor may do to fix what the check finds.
    pub fix_class: FixClass,
}

/// What a check may look at during a run.
#[derive(Debug, Clone)]
pub struct Context {
    /// Files of the inspected system.
    pub fs: RootFs,
    /// The user's home directory as a path inside the root, when given with
    /// `--home`. Resolving it from the passwd database is planned.
    pub home: Option<PathBuf>,
    /// The user name given with `--user`.
    pub user: Option<String>,
}

impl Context {
    /// A context with no user information.
    pub fn new(fs: RootFs) -> Self {
        Context {
            fs,
            home: None,
            user: None,
        }
    }
}

/// Why a check could not finish.
#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    /// A file could not be read.
    #[error(transparent)]
    Fs(#[from] FsError),
    /// Anything else.
    #[error("{0}")]
    Other(String),
}

/// One diagnostic check.
///
/// A check reads facts through the [`Context`] only and returns findings. It
/// never changes anything; fixes are proposals attached to findings.
pub trait Check: Send + Sync {
    /// The check's static description.
    fn info(&self) -> CheckInfo;

    /// Runs the check. An empty vector means nothing was found.
    fn run(&self, cx: &Context) -> Result<Vec<Finding>, CheckError>;
}

/// Which checks a run should include. Empty lists mean "all".
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Selection {
    /// Only these ids.
    pub ids: Vec<CheckId>,
    /// Only these areas.
    pub areas: Vec<Area>,
    /// Only checks that apply to this session.
    pub session: Option<Session>,
}

impl Selection {
    /// Whether a check is selected.
    pub fn matches(&self, info: &CheckInfo) -> bool {
        (self.ids.is_empty() || self.ids.contains(&info.id))
            && (self.areas.is_empty() || self.areas.contains(&info.id.area()))
            && self.session.is_none_or(|s| info.sessions.contains(&s))
    }
}

/// The set of known checks.
#[derive(Default)]
pub struct Registry {
    checks: Vec<Box<dyn Check>>,
}

impl std::fmt::Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.checks.iter().map(|c| c.info().id))
            .finish()
    }
}

impl Registry {
    /// An empty registry.
    pub fn new() -> Self {
        Registry::default()
    }

    /// Adds a check, keeping the registry sorted by id.
    ///
    /// # Panics
    ///
    /// Panics if a check with the same id is already registered.
    pub fn register(&mut self, check: Box<dyn Check>) {
        let id = check.info().id;
        match self.checks.binary_search_by_key(&id, |c| c.info().id) {
            Ok(_) => panic!("check {id} registered twice"),
            Err(pos) => self.checks.insert(pos, check),
        }
    }

    /// Looks a check up by id.
    pub fn get(&self, id: CheckId) -> Option<&dyn Check> {
        self.checks
            .iter()
            .find(|c| c.info().id == id)
            .map(|c| c.as_ref())
    }

    /// All checks, sorted by id.
    pub fn iter(&self) -> impl Iterator<Item = &dyn Check> {
        self.checks.iter().map(|c| c.as_ref())
    }

    /// Runs the selected checks and collects a finished [`Report`].
    pub fn run(&self, cx: &Context, selection: &Selection) -> Report {
        let mut report = Report::default();
        for check in self.iter().filter(|c| selection.matches(&c.info())) {
            let id = check.info().id;
            report.checks.push(id);
            match check.run(cx) {
                Ok(findings) => report.findings.extend(findings),
                Err(e) => report.failures.push(CheckFailure {
                    id,
                    message: e.to_string(),
                }),
            }
        }
        report.finish();
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finding::Severity;

    struct Fake {
        id: CheckId,
        sessions: &'static [Session],
        fail: bool,
    }

    impl Check for Fake {
        fn info(&self) -> CheckInfo {
            CheckInfo {
                id: self.id,
                title: "fake",
                sessions: self.sessions,
                fix_class: FixClass::ReportOnly,
            }
        }

        fn run(&self, _cx: &Context) -> Result<Vec<Finding>, CheckError> {
            if self.fail {
                return Err(CheckError::Other("broken".into()));
            }
            Ok(vec![Finding::new(self.id, Severity::Warning, "found")])
        }
    }

    fn registry() -> Registry {
        let mut r = Registry::new();
        r.register(Box::new(Fake {
            id: CheckId::new(Area::Pwr, 1),
            sessions: &[Session::Sway],
            fail: false,
        }));
        r.register(Box::new(Fake {
            id: CheckId::new(Area::Apt, 1),
            sessions: &[Session::I3, Session::Sway],
            fail: false,
        }));
        r.register(Box::new(Fake {
            id: CheckId::new(Area::Apt, 2),
            sessions: &[Session::I3, Session::Sway],
            fail: true,
        }));
        r
    }

    fn context() -> Context {
        Context::new(RootFs::new(env!("CARGO_MANIFEST_DIR")).unwrap())
    }

    #[test]
    fn checks_are_kept_sorted() {
        let ids: Vec<String> = registry().iter().map(|c| c.info().id.to_string()).collect();
        assert_eq!(ids, ["APT-001", "APT-002", "PWR-001"]);
    }

    #[test]
    fn selection_filters_by_id_area_and_session() {
        let r = registry();
        let only_sway_pwr = Selection {
            areas: vec![Area::Pwr],
            ..Selection::default()
        };
        assert_eq!(r.run(&context(), &only_sway_pwr).checks.len(), 1);
        let i3 = Selection {
            session: Some(Session::I3),
            ..Selection::default()
        };
        let report = r.run(&context(), &i3);
        assert_eq!(report.checks.len(), 2);
        let one = Selection {
            ids: vec![CheckId::new(Area::Apt, 1)],
            ..Selection::default()
        };
        assert_eq!(r.run(&context(), &one).findings.len(), 1);
    }

    #[test]
    fn a_failing_check_does_not_stop_the_others() {
        let report = registry().run(&context(), &Selection::default());
        assert_eq!(report.checks.len(), 3);
        assert_eq!(report.findings.len(), 2);
        assert_eq!(report.failures.len(), 1);
        assert_eq!(report.failures[0].id, CheckId::new(Area::Apt, 2));
    }

    #[test]
    #[should_panic(expected = "registered twice")]
    fn duplicate_ids_panic() {
        let mut r = registry();
        r.register(Box::new(Fake {
            id: CheckId::new(Area::Apt, 1),
            sessions: &[],
            fail: false,
        }));
    }
}

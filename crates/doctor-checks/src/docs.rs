//! Text of `docs/checks/<ID>.md` for every catalogued check, compiled in for
//! the `explain` command. Adding a check means adding its file and a line
//! here; a test keeps the list and the directory in sync.

use doctor_core::CheckId;

macro_rules! docs {
    ($($id:literal),* $(,)?) => {
        /// `(id, markdown)` for every documented check, sorted by id.
        const DOCS: &[(&str, &str)] = &[
            $(($id, include_str!(concat!("../../../docs/checks/", $id, ".md"))),)*
        ];
    };
}

docs![
    "APT-001",
    "APT-002",
    "APT-003",
    "APT-004",
    "APT-005",
    "APT-006",
    "APT-007",
    "APT-008",
    "APT-009",
    "APT-010",
    "BAR-001",
    "BAR-002",
    "BAR-003",
    "BAR-004",
    "BAR-005",
    "CFG-001",
    "CFG-002",
    "CFG-003",
    "CFG-004",
    "CFG-005",
    "CFG-006",
    "CFG-007",
    "CFG-008",
    "CFG-009",
    "CFG-010",
    "CFG-011",
    "CFG-012",
    "CFG-013",
    "CFG-014",
    "CFG-015",
    "COMP-001",
    "COMP-002",
    "COMP-003",
    "COMP-004",
    "DISP-001",
    "DISP-002",
    "DISP-003",
    "DISP-004",
    "ENV-001",
    "INPUT-001",
    "KBD-001",
    "KBD-002",
    "KBD-003",
    "KBD-004",
    "KBD-005",
    "LNCH-001",
    "LNCH-002",
    "LNCH-003",
    "LOOK-001",
    "LOOK-002",
    "LOOK-003",
    "LOOK-004",
    "LOOK-005",
    "LOOK-006",
    "NOTIF-001",
    "NOTIF-002",
    "PKG-001",
    "PKG-002",
    "PKG-003",
    "PKG-004",
    "PKG-005",
    "PKG-006",
    "PKG-007",
    "PKG-008",
    "PKG-009",
    "PKG-010",
    "PKG-011",
    "PORT-001",
    "PORT-002",
    "PWR-001",
    "PWR-002",
    "PWR-003",
    "SES-001",
    "SES-002",
    "SES-003",
    "SES-004",
    "SES-005",
    "SES-006",
    "SES-007",
    "SES-008",
    "SES-009",
    "SES-010",
    "XRES-001",
    "XRES-002",
    "XRES-003",
    "XRES-004",
    "XRES-005",
    "XRES-006",
    "XRES-007",
    "XRES-008",
    "XRES-009",
];

/// The markdown documentation of a check, if it is catalogued.
pub fn doc(id: CheckId) -> Option<&'static str> {
    let key = id.to_string();
    DOCS.iter().find(|(k, _)| *k == key).map(|(_, text)| *text)
}

/// Every catalogued check id, sorted.
pub fn documented() -> impl Iterator<Item = CheckId> {
    DOCS.iter()
        .map(|(id, _)| id.parse().expect("docs list holds valid ids"))
}

/// The title from a doc's first line (`# ID: title`).
pub fn title(doc: &str) -> Option<&str> {
    let first = doc.lines().next()?.strip_prefix("# ")?;
    first.split_once(": ").map(|(_, title)| title)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

    use super::*;

    #[test]
    fn list_matches_the_docs_directory() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/checks");
        let on_disk: BTreeSet<String> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .filter_map(|n| n.strip_suffix(".md").map(str::to_owned))
            .collect();
        let listed: BTreeSet<String> = DOCS.iter().map(|(id, _)| (*id).to_owned()).collect();
        assert_eq!(on_disk, listed);
    }

    #[test]
    fn every_doc_has_the_standard_shape() {
        for (key, text) in DOCS {
            let id: CheckId = key.parse().unwrap();
            assert!(text.starts_with(&format!("# {id}: ")), "{key}: heading");
            assert!(title(text).is_some_and(|t| !t.is_empty()), "{key}: title");
            for section in [
                "## What it detects",
                "## How it is detected",
                "## Fix",
                "## Sources",
            ] {
                assert!(text.contains(section), "{key}: missing {section}");
            }
            assert!(
                text.contains(&format!("| Area | {} |", id.area())),
                "{key}: area"
            );
        }
    }

    #[test]
    fn status_says_implemented_exactly_for_registered_checks() {
        let registry = crate::registry();
        for id in documented() {
            let implemented = registry.get(id).is_some();
            let expected = if implemented {
                "| Status | implemented |"
            } else {
                "| Status | planned |"
            };
            assert!(doc(id).unwrap().contains(expected), "{id}: {expected}");
        }
        for check in registry.iter() {
            assert!(
                doc(check.info().id).is_some(),
                "{} has no doc",
                check.info().id
            );
        }
    }

    #[test]
    fn docs_are_sorted_and_unique() {
        let ids: Vec<CheckId> = documented().collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(ids, sorted);
        assert_eq!(ids.len(), 91);
    }
}

//! ENV: the platform Regolith runs on.

use doctor_core::os_release;
use doctor_core::{
    Area, Check, CheckError, CheckId, CheckInfo, Context, Evidence, Finding, FixClass, Session,
    Severity,
};

use crate::data;

/// ENV-001: platform Regolith does not support.
///
/// Reports a distribution that is not Ubuntu or Debian based, Windows
/// Subsystem for Linux, and a CPU architecture the archive does not build.
/// See `docs/checks/ENV-001.md`.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnsupportedPlatform;

impl UnsupportedPlatform {
    /// The check's id.
    pub const ID: CheckId = CheckId::new(Area::Env, 1);
    const SESSIONS: &'static [Session] = &[Session::I3, Session::Sway];
}

/// Files that carry the kernel version string, in the order they are read.
const KERNEL_VERSION_FILES: [&str; 2] = ["/proc/version", "/proc/sys/kernel/osrelease"];
/// File with the kernel's machine name.
const KERNEL_ARCH_FILE: &str = "/proc/sys/kernel/arch";

impl Check for UnsupportedPlatform {
    fn info(&self) -> CheckInfo {
        CheckInfo {
            id: Self::ID,
            title: "Platform Regolith does not support",
            sessions: Self::SESSIONS,
            fix_class: FixClass::ReportOnly,
        }
    }

    fn run(&self, cx: &Context) -> Result<Vec<Finding>, CheckError> {
        let table = data::platforms();
        let mut findings = Vec::new();
        let finding = |severity, message: String, evidence: Evidence, fix: String| {
            Finding::new(Self::ID, severity, message)
                .with_sessions(Self::SESSIONS)
                .with_evidence(evidence)
                .with_fix(FixClass::ReportOnly, fix)
        };

        match os_release::read(&cx.fs)? {
            None => findings.push(finding(
                Severity::Info,
                "no os-release file; cannot tell which distribution this is".to_owned(),
                Evidence::file(os_release::PATHS[0]),
                "Check that the system is Ubuntu or Debian; Regolith publishes packages only for those."
                    .to_owned(),
            )),
            Some(osr) => {
                let id = osr.id();
                let supported = std::iter::once(&id)
                    .chain(osr.id_like().iter())
                    .any(|d| table.distributions.contains(d));
                if !supported {
                    findings.push(finding(
                        Severity::Error,
                        format!(
                            "{} is not Ubuntu or Debian and not derived from them",
                            osr.display_name()
                        ),
                        Evidence::file(osr.path).with_note(format!("ID={id}")),
                        format!(
                            "Regolith publishes packages only for {}; use one of those or a derivative of them.",
                            table.distributions.join(" and ")
                        ),
                    ));
                }
            }
        }

        for path in KERNEL_VERSION_FILES {
            let Some(text) = cx.fs.read_to_string(path)? else {
                continue;
            };
            if text.to_ascii_lowercase().contains("microsoft") {
                findings.push(finding(
                    Severity::Warning,
                    "running under Windows Subsystem for Linux, which Regolith does not support"
                        .to_owned(),
                    Evidence::file(path).with_note(text.trim().to_owned()),
                    "Install Regolith on a real or virtual machine; under WSL the session, display server and input stack differ from what Regolith expects."
                        .to_owned(),
                ));
            }
            // The first file that exists is enough.
            break;
        }

        if let Some(text) = cx.fs.read_to_string(KERNEL_ARCH_FILE)? {
            let machine = text.trim();
            if !machine.is_empty() && !table.architectures.iter().any(|a| a.kernel == machine) {
                let names: Vec<&str> = table
                    .architectures
                    .iter()
                    .map(|a| a.debian.as_str())
                    .collect();
                findings.push(finding(
                    Severity::Error,
                    format!("Regolith has no packages for the {machine} architecture"),
                    Evidence::file(KERNEL_ARCH_FILE).with_note(machine.to_owned()),
                    format!(
                        "Regolith publishes packages only for {}.",
                        names.join(" and ")
                    ),
                ));
            }
        }

        Ok(findings)
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use doctor_core::RootFs;

    use super::*;

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/env-001")
            .join(name)
    }

    fn run(name: &str) -> Vec<Finding> {
        let cx = Context::new(RootFs::new(fixture(name)).unwrap());
        UnsupportedPlatform.run(&cx).unwrap()
    }

    fn summary(findings: &[Finding]) -> Vec<(Severity, Option<&str>)> {
        findings
            .iter()
            .map(|f| (f.severity, f.evidence[0].path.as_deref()))
            .collect()
    }

    #[test]
    fn supported_ubuntu_is_clean() {
        assert!(run("ubuntu-noble").is_empty());
    }

    #[test]
    fn debian_from_usr_lib_os_release_on_arm64_is_clean() {
        assert!(run("debian-trixie-arm64").is_empty());
    }

    #[test]
    fn derivative_is_left_to_apt_007() {
        assert!(run("linuxmint").is_empty());
    }

    #[test]
    fn non_debian_distribution_is_an_error() {
        let findings = run("arch");
        assert_eq!(
            summary(&findings),
            [(Severity::Error, Some("/etc/os-release"))]
        );
        assert!(findings[0].message.contains("Arch Linux"));
        assert_eq!(findings[0].evidence[0].note.as_deref(), Some("ID=arch"));
        assert_eq!(findings[0].sessions, [Session::I3, Session::Sway]);
        assert_eq!(findings[0].fixes[0].class, FixClass::ReportOnly);
    }

    #[test]
    fn wsl_kernel_is_a_warning() {
        assert_eq!(
            summary(&run("wsl")),
            [(Severity::Warning, Some("/proc/version"))]
        );
    }

    #[test]
    fn unsupported_architecture_is_an_error() {
        let findings = run("riscv64");
        assert_eq!(
            summary(&findings),
            [(Severity::Error, Some("/proc/sys/kernel/arch"))]
        );
        assert!(findings[0].message.contains("riscv64"));
    }

    #[test]
    fn missing_os_release_is_info() {
        assert_eq!(
            summary(&run("empty")),
            [(Severity::Info, Some("/etc/os-release"))]
        );
    }

    #[test]
    fn info_matches_the_docs() {
        let info = UnsupportedPlatform.info();
        let doc = crate::docs::doc(info.id).unwrap();
        assert!(doc.starts_with(&format!("# {}: {}\n", info.id, info.title)));
    }
}

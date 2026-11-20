//! End-to-end tests of the `regolith-doctor` binary against fixture trees.

use std::path::PathBuf;
use std::process::{Command, Output};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/env-001")
        .join(name)
}

fn doctor(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_regolith-doctor"))
        .args(args)
        .output()
        .expect("binary runs")
}

fn check_json(root: &str) -> (i32, serde_json::Value) {
    let root = fixture(root);
    let out = doctor(&["--root", root.to_str().unwrap(), "--json", "check"]);
    let json = serde_json::from_slice(&out.stdout).expect("valid JSON on stdout");
    (out.status.code().unwrap(), json)
}

#[test]
fn clean_system_exits_zero_with_an_empty_report() {
    let (code, json) = check_json("ubuntu-noble");
    assert_eq!(code, 0);
    assert_eq!(json["schema_version"], 1);
    assert_eq!(json["checks"], serde_json::json!(["ENV-001"]));
    assert_eq!(json["findings"], serde_json::json!([]));
}

#[test]
fn unsupported_distribution_exits_two() {
    let (code, json) = check_json("arch");
    assert_eq!(code, 2);
    let finding = &json["findings"][0];
    assert_eq!(finding["id"], "ENV-001");
    assert_eq!(finding["severity"], "error");
    assert_eq!(finding["sessions"], serde_json::json!(["i3", "sway"]));
    assert_eq!(finding["evidence"][0]["path"], "/etc/os-release");
    assert_eq!(finding["fixes"][0]["class"], "report-only");
}

#[test]
fn wsl_exits_one() {
    let (code, json) = check_json("wsl");
    assert_eq!(code, 1);
    assert_eq!(json["findings"][0]["severity"], "warning");
}

#[test]
fn text_output_names_the_check_and_the_file() {
    let root = fixture("riscv64");
    let out = doctor(&["check", "--root", root.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(2));
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.contains("ENV-001 error [i3, sway]:"), "{text}");
    assert!(text.contains("at /proc/sys/kernel/arch"), "{text}");
    assert!(
        text.contains("1 check run: 1 error, 0 warnings, 0 info"),
        "{text}"
    );
}

#[test]
fn filters_that_exclude_every_check_give_an_empty_run() {
    let root = fixture("arch");
    let out = doctor(&["check", "--root", root.to_str().unwrap(), "--area", "apt"]);
    assert_eq!(out.status.code(), Some(0));
    assert!(
        String::from_utf8(out.stdout)
            .unwrap()
            .contains("0 checks run")
    );
}

#[test]
fn asking_for_a_planned_check_is_not_implemented() {
    let root = fixture("ubuntu-noble");
    let out = doctor(&["check", "--root", root.to_str().unwrap(), "--id", "APT-001"]);
    assert_eq!(out.status.code(), Some(4));
    let out = doctor(&["check", "--root", root.to_str().unwrap(), "--id", "APT-999"]);
    assert_eq!(out.status.code(), Some(3));
}

#[test]
fn explain_prints_the_check_doc() {
    let out = doctor(&["explain", "env-001"]);
    assert_eq!(out.status.code(), Some(0));
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.starts_with("# ENV-001: Platform Regolith does not support"));
    assert!(!text.contains("does not run this check yet"));

    let out = doctor(&["explain", "CFG-001"]);
    assert_eq!(out.status.code(), Some(0));
    assert!(
        String::from_utf8(out.stdout)
            .unwrap()
            .contains("does not run this check yet")
    );
}

#[test]
fn explain_json_has_title_and_status() {
    let out = doctor(&["--json", "explain", "ENV-001"]);
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["id"], "ENV-001");
    assert_eq!(json["title"], "Platform Regolith does not support");
    assert_eq!(json["implemented"], true);
}

#[test]
fn explain_rejects_unknown_ids_and_defers_topics() {
    assert_eq!(doctor(&["explain", "ENV-099"]).status.code(), Some(3));
    assert_eq!(doctor(&["explain", "bar font"]).status.code(), Some(4));
}

#[test]
fn unimplemented_commands_exit_four() {
    for args in [
        &["keys"][..],
        &["config"],
        &["fix", "--dry-run"],
        &["undo"],
        &["history"],
        &["validate"],
        &["snapshot", "save", "/nonexistent"],
        &["report", "--redact"],
        &["--snapshot", "/nonexistent", "check"],
    ] {
        let out = doctor(args);
        assert_eq!(out.status.code(), Some(4), "{args:?}");
        assert!(
            String::from_utf8(out.stderr)
                .unwrap()
                .contains("not implemented yet"),
            "{args:?}"
        );
    }
}

#[test]
fn usage_errors_exit_three_and_help_exits_zero() {
    assert_eq!(doctor(&["frobnicate"]).status.code(), Some(3));
    assert_eq!(doctor(&["check", "--id", "nope"]).status.code(), Some(3));
    assert_eq!(
        doctor(&["--root", "/nonexistent-root", "check"])
            .status
            .code(),
        Some(3)
    );
    assert_eq!(doctor(&["--help"]).status.code(), Some(0));
    assert_eq!(doctor(&["--version"]).status.code(), Some(0));
}

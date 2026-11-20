//! `regolith-doctor`: diagnose Regolith Desktop problems.
//!
//! This binary parses the command line and prints results; checks and core
//! types live in the `doctor-checks` and `doctor-core` crates.

mod cli;

use std::io::Write;
use std::process::ExitCode;

use clap::Parser;
use doctor_core::{CheckId, Context, ExitStatus, Report, RootFs, Selection, Severity};
use serde::Serialize;

use cli::{Cli, Command};

/// Why a command did not produce a normal result.
#[derive(Debug)]
enum Failure {
    /// The command or option exists but is not implemented yet.
    NotImplemented(String),
    /// Bad input or an internal error.
    Error(String),
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            // Help and version go to stdout and exit 0; real usage errors
            // exit with the doctor's usage status.
            let _ = e.print();
            return if e.use_stderr() {
                ExitCode::from(ExitStatus::Failure.code())
            } else {
                ExitCode::SUCCESS
            };
        }
    };
    let status = match run(&cli) {
        Ok(status) => status,
        Err(Failure::NotImplemented(what)) => {
            eprintln!("regolith-doctor: {what} is not implemented yet");
            ExitStatus::NotImplemented
        }
        Err(Failure::Error(message)) => {
            eprintln!("regolith-doctor: {message}");
            ExitStatus::Failure
        }
    };
    ExitCode::from(status.code())
}

fn run(cli: &Cli) -> Result<ExitStatus, Failure> {
    let global = &cli.global;
    if global.snapshot.is_some() {
        return Err(Failure::NotImplemented("--snapshot".into()));
    }
    match &cli.command {
        Command::Check { area, id, session } => {
            let fs = RootFs::new(&global.root).map_err(|e| Failure::Error(e.to_string()))?;
            let cx = Context {
                fs,
                home: global.home.clone(),
                user: global.user.clone(),
            };
            let selection = Selection {
                ids: id.clone(),
                areas: area.clone(),
                session: session.session(),
            };
            let registry = doctor_checks::registry();
            for wanted in id {
                if registry.get(*wanted).is_none() {
                    return Err(match doctor_checks::docs::doc(*wanted) {
                        Some(_) => Failure::NotImplemented(format!("check {wanted}")),
                        None => Failure::Error(format!("unknown check {wanted}")),
                    });
                }
            }
            let report = registry.run(&cx, &selection);
            if global.json {
                print_json(&report)?;
            } else {
                print_text(&report);
            }
            Ok(report.exit_status())
        }
        Command::Explain { topic } => explain(topic, global.json),
        other => Err(Failure::NotImplemented(format!("`{}`", other.name()))),
    }
}

fn explain(topic: &str, json: bool) -> Result<ExitStatus, Failure> {
    let id: CheckId = topic
        .parse()
        .map_err(|_| Failure::NotImplemented("explaining topics other than check ids".into()))?;
    let doc = doctor_checks::docs::doc(id)
        .ok_or_else(|| Failure::Error(format!("unknown check {id}")))?;
    let implemented = doctor_checks::registry().get(id).is_some();
    if json {
        #[derive(Serialize)]
        struct Explanation<'a> {
            id: CheckId,
            title: &'a str,
            implemented: bool,
            text: &'a str,
        }
        print_json(&Explanation {
            id,
            title: doctor_checks::docs::title(doc).unwrap_or_default(),
            implemented,
            text: doc,
        })?;
    } else {
        print!("{doc}");
        if !implemented {
            println!("\nNote: regolith-doctor does not run this check yet.");
        }
    }
    Ok(ExitStatus::Clean)
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Failure> {
    let mut out = std::io::stdout().lock();
    serde_json::to_writer_pretty(&mut out, value)
        .map_err(|e| Failure::Error(format!("cannot write JSON: {e}")))?;
    writeln!(out).map_err(|e| Failure::Error(format!("cannot write output: {e}")))
}

fn print_text(report: &Report) {
    for finding in &report.findings {
        let sessions: Vec<String> = finding.sessions.iter().map(ToString::to_string).collect();
        println!(
            "{} {} [{}]: {}",
            finding.id,
            finding.severity,
            sessions.join(", "),
            finding.message
        );
        for evidence in &finding.evidence {
            if let Some(path) = &evidence.path {
                match evidence.line {
                    Some(line) => println!("  at {path}:{line}"),
                    None => println!("  at {path}"),
                }
            }
            if let Some(note) = &evidence.note {
                println!("  found: {note}");
            }
        }
        for fix in &finding.fixes {
            println!("  fix ({}): {}", fix.class, fix.summary);
        }
    }
    for failure in &report.failures {
        println!("{} failed to run: {}", failure.id, failure.message);
    }
    let plural = |n: usize, word: &str| format!("{n} {word}{}", if n == 1 { "" } else { "s" });
    println!(
        "{} run: {}, {}, {} info{}",
        plural(report.checks.len(), "check"),
        plural(report.count(Severity::Error), "error"),
        plural(report.count(Severity::Warning), "warning"),
        report.count(Severity::Info),
        if report.failures.is_empty() {
            String::new()
        } else {
            format!(", {} failed", report.failures.len())
        }
    );
}

//! Command-line definition.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use doctor_core::{Area, CheckId, Session};

/// Diagnose Regolith Desktop problems and fix them where it is safe.
#[derive(Debug, Parser)]
#[command(name = "regolith-doctor", version, about, propagate_version = true)]
pub struct Cli {
    /// Global options.
    #[command(flatten)]
    pub global: Global,
    /// What to do.
    #[command(subcommand)]
    pub command: Command,
}

/// Options shared by every command.
#[derive(Debug, Args)]
pub struct Global {
    /// Inspect the system rooted at DIR; every file is read relative to it
    #[arg(long, global = true, value_name = "DIR", default_value = "/")]
    pub root: PathBuf,
    /// The user's home directory, as a path inside the root
    #[arg(long, global = true, value_name = "DIR")]
    pub home: Option<PathBuf>,
    /// The user whose configuration is inspected
    #[arg(long, global = true, value_name = "NAME")]
    pub user: Option<String>,
    /// Replay command and IPC output recorded with `snapshot save` (not implemented yet)
    #[arg(long, global = true, value_name = "DIR")]
    pub snapshot: Option<PathBuf>,
    /// Print machine-readable JSON instead of text
    #[arg(long, global = true)]
    pub json: bool,
    /// Never use colour in text output
    #[arg(long, global = true)]
    pub no_color: bool,
}

/// Session selector used by several commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SessionArg {
    /// The X11 session (i3).
    I3,
    /// The Wayland session (sway).
    Sway,
    /// Both sessions.
    Both,
}

impl SessionArg {
    /// The single session to filter on, or `None` for both.
    pub fn session(self) -> Option<Session> {
        match self {
            SessionArg::I3 => Some(Session::I3),
            SessionArg::Sway => Some(Session::Sway),
            SessionArg::Both => None,
        }
    }
}

/// Subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run checks and print findings
    Check {
        /// Only checks in this area (repeatable), for example `apt`
        #[arg(long, value_name = "AREA")]
        area: Vec<Area>,
        /// Only this check (repeatable), for example `ENV-001`
        #[arg(long, value_name = "ID")]
        id: Vec<CheckId>,
        /// Only checks for this session
        #[arg(long, value_enum, default_value_t = SessionArg::Both)]
        session: SessionArg,
    },
    /// Explain a check, or answer a "how do I change X" topic
    Explain {
        /// A check id such as `ENV-001`, or a topic
        topic: String,
    },
    /// Show effective Xresources values and where each one was set (not implemented yet)
    Keys {
        /// Session to resolve for
        #[arg(long, value_enum, default_value_t = SessionArg::Both)]
        session: SessionArg,
        /// Only this key
        key: Option<String>,
    },
    /// Show the effective window manager partials and bindings (not implemented yet)
    Config {
        /// Session to resolve for
        #[arg(long, value_enum, default_value_t = SessionArg::Both)]
        session: SessionArg,
    },
    /// Apply fixes for findings (not implemented yet)
    Fix {
        /// Show what would change without changing anything
        #[arg(long)]
        dry_run: bool,
        /// Only fixes for this check (repeatable)
        #[arg(long, value_name = "ID")]
        id: Vec<CheckId>,
        /// Apply guided fixes without asking
        #[arg(long)]
        yes: bool,
    },
    /// Undo an applied fix transaction (not implemented yet)
    Undo {
        /// Transaction id; the latest when omitted
        txid: Option<String>,
    },
    /// List applied fix transactions (not implemented yet)
    History,
    /// Ask i3 and sway themselves to validate the configuration (not implemented yet)
    Validate {
        /// Session to validate
        #[arg(long, value_enum, default_value_t = SessionArg::Both)]
        session: SessionArg,
    },
    /// Record command and IPC output for later replay (not implemented yet)
    Snapshot {
        /// Snapshot action.
        #[command(subcommand)]
        action: SnapshotAction,
    },
    /// Write a support bundle for bug reports (not implemented yet)
    Report {
        /// Remove user names, home paths, host names and similar data
        #[arg(long)]
        redact: bool,
        /// Write to FILE instead of standard output
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

/// `snapshot` subcommands.
#[derive(Debug, Subcommand)]
pub enum SnapshotAction {
    /// Save a snapshot into DIR
    Save {
        /// Directory to write
        dir: PathBuf,
    },
}

impl Command {
    /// The command's name, for messages.
    pub fn name(&self) -> &'static str {
        match self {
            Command::Check { .. } => "check",
            Command::Explain { .. } => "explain",
            Command::Keys { .. } => "keys",
            Command::Config { .. } => "config",
            Command::Fix { .. } => "fix",
            Command::Undo { .. } => "undo",
            Command::History => "history",
            Command::Validate { .. } => "validate",
            Command::Snapshot { .. } => "snapshot save",
            Command::Report { .. } => "report",
        }
    }
}

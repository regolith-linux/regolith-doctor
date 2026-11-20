//! Core types shared by every part of regolith-doctor.
//!
//! The pieces are deliberately small:
//!
//! - [`id`]: stable check identifiers such as `ENV-001`.
//! - [`finding`]: severities, sessions, findings and fix proposals.
//! - [`report`]: the result of one run, its JSON form and its exit code.
//! - [`registry`]: the [`Check`](registry::Check) trait and the registry that runs checks.
//! - [`rootfs`]: all file access, relative to the `--root` directory.
//! - [`os_release`]: a minimal reader for `os-release` files.
//!
//! Checks never read the live system directly; they go through
//! [`rootfs::RootFs`] so that the same code runs against `/` and against a
//! fixture tree.

pub mod finding;
pub mod id;
pub mod os_release;
pub mod registry;
pub mod report;
pub mod rootfs;

pub use finding::{Evidence, Finding, FixClass, FixProposal, Session, Severity};
pub use id::{Area, CheckId, ParseIdError};
pub use registry::{Check, CheckError, CheckInfo, Context, Registry, Selection};
pub use report::{ExitStatus, Report};
pub use rootfs::{FsError, RootFs};

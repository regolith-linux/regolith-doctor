//! The checks of regolith-doctor.
//!
//! Each area has a module; each check is a type implementing
//! [`doctor_core::Check`]. [`registry`] returns every implemented check.
//! [`docs`] holds the text of `docs/checks/<ID>.md` for every catalogued
//! check, implemented or not, for the `explain` command.

pub mod data;
pub mod docs;
pub mod env;

use doctor_core::Registry;

/// A registry with every implemented check.
pub fn registry() -> Registry {
    let mut registry = Registry::new();
    registry.register(Box::new(env::UnsupportedPlatform));
    registry
}

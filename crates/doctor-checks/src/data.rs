//! Data tables from the repository's `data/` directory, compiled into the
//! binary so the doctor works offline and needs no installed data files.

use std::sync::LazyLock;

use serde::Deserialize;

/// `data/platforms.toml`: distributions and architectures Regolith supports.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Platforms {
    /// Supported `os-release` IDs.
    pub distributions: Vec<String>,
    /// Supported architectures.
    pub architectures: Vec<Architecture>,
}

/// One supported architecture.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Architecture {
    /// Kernel machine name, such as `x86_64`.
    pub kernel: String,
    /// Debian architecture name, such as `amd64`.
    pub debian: String,
}

const PLATFORMS_TOML: &str = include_str!("../../../data/platforms.toml");

/// The parsed platforms table.
pub fn platforms() -> &'static Platforms {
    static TABLE: LazyLock<Platforms> = LazyLock::new(|| {
        toml::from_str(PLATFORMS_TOML).expect("data/platforms.toml is valid (checked by tests)")
    });
    &TABLE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platforms_table_parses() {
        let p = platforms();
        assert_eq!(p.distributions, ["ubuntu", "debian"]);
        let debian: Vec<&str> = p.architectures.iter().map(|a| a.debian.as_str()).collect();
        assert_eq!(debian, ["amd64", "arm64"]);
    }
}

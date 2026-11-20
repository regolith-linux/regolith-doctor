//! Minimal reader for `os-release` files.
//!
//! It handles `KEY=value` lines, comments, blank lines and one pair of
//! matching single or double quotes around a value. It does not yet handle
//! backslash escapes or other shell quoting; a full parser that follows
//! `os-release(5)` is planned.

use std::collections::BTreeMap;

use crate::rootfs::{FsError, RootFs};

/// Where `os-release` lives, in lookup order (see `os-release(5)`).
pub const PATHS: [&str; 2] = ["/etc/os-release", "/usr/lib/os-release"];

/// A parsed `os-release` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsRelease {
    /// The path (inside the root) it was read from.
    pub path: &'static str,
    /// All fields.
    pub fields: BTreeMap<String, String>,
}

impl OsRelease {
    /// A field's value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(String::as_str)
    }

    /// `ID`, lower-cased; `linux` when absent, as the spec says.
    pub fn id(&self) -> String {
        self.get("ID").unwrap_or("linux").to_ascii_lowercase()
    }

    /// The space-separated `ID_LIKE` list, lower-cased.
    pub fn id_like(&self) -> Vec<String> {
        self.get("ID_LIKE")
            .unwrap_or("")
            .split_whitespace()
            .map(str::to_ascii_lowercase)
            .collect()
    }

    /// `PRETTY_NAME`, else `NAME`, else `ID`.
    pub fn display_name(&self) -> String {
        self.get("PRETTY_NAME")
            .or_else(|| self.get("NAME"))
            .map(str::to_owned)
            .unwrap_or_else(|| self.id())
    }
}

/// Parses the text of an `os-release` file.
pub fn parse(text: &str) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = value.trim();
        let unquoted = b"\"'"
            .iter()
            .find_map(|&q| {
                let bytes = value.as_bytes();
                (bytes.len() >= 2 && bytes[0] == q && bytes[bytes.len() - 1] == q)
                    .then(|| &value[1..value.len() - 1])
            })
            .unwrap_or(value);
        fields.insert(key.trim().to_owned(), unquoted.to_owned());
    }
    fields
}

/// Reads the first `os-release` file found under the root.
pub fn read(fs: &RootFs) -> Result<Option<OsRelease>, FsError> {
    for path in PATHS {
        if let Some(text) = fs.read_to_string(path)? {
            return Ok(Some(OsRelease {
                path,
                fields: parse(&text),
            }));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_and_bare_values() {
        let fields = parse(
            "# comment\nNAME=\"Ubuntu\"\nID=ubuntu\nID_LIKE='debian'\n\nVERSION_CODENAME=noble\nbroken line\n",
        );
        assert_eq!(fields["NAME"], "Ubuntu");
        assert_eq!(fields["ID"], "ubuntu");
        assert_eq!(fields["ID_LIKE"], "debian");
        assert_eq!(fields["VERSION_CODENAME"], "noble");
        assert_eq!(fields.len(), 4);
    }

    #[test]
    fn id_defaults_to_linux_and_lowercases() {
        let osr = OsRelease {
            path: PATHS[0],
            fields: parse("ID_LIKE=\"Ubuntu Debian\"\n"),
        };
        assert_eq!(osr.id(), "linux");
        assert_eq!(osr.id_like(), vec!["ubuntu", "debian"]);
        assert_eq!(osr.display_name(), "linux");
    }
}

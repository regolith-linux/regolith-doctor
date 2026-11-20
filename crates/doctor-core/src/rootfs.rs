//! File access relative to the `--root` directory.
//!
//! Every check reads files through [`RootFs`]. Callers always use the
//! absolute path the file would have on the inspected system
//! (`/etc/os-release`); `RootFs` maps it under the root directory. With the
//! default root `/` this is the live system; with a fixture tree it is that
//! tree.
//!
//! Current limits, to be lifted by the snapshot layer:
//!
//! - Paths with `..` components are rejected rather than resolved.
//! - Symbolic links are followed the way the host follows them. An absolute
//!   link inside a mounted system image therefore points at the host, not
//!   into the image. Chroot-like link resolution (absolute targets resolved
//!   inside the root, nothing read outside it) is planned.

use std::io;
use std::path::{Component, Path, PathBuf};

/// Errors from [`RootFs`].
#[derive(Debug, thiserror::Error)]
pub enum FsError {
    /// The root directory does not exist or is not a directory.
    #[error("root `{0}` is not a directory")]
    BadRoot(PathBuf),
    /// Callers must pass absolute paths.
    #[error("`{0}` is not an absolute path")]
    NotAbsolute(PathBuf),
    /// The path contains `..` or another component that could leave the root.
    #[error("`{0}` contains a component that is not allowed (`..` or a prefix)")]
    Forbidden(PathBuf),
    /// Reading failed for a reason other than the file being absent.
    #[error("cannot read `{path}`: {source}")]
    Io {
        /// The path as seen inside the root.
        path: PathBuf,
        /// The underlying error.
        #[source]
        source: io::Error,
    },
}

/// Read-only view of a filesystem tree rooted at a directory.
#[derive(Debug, Clone)]
pub struct RootFs {
    root: PathBuf,
}

impl RootFs {
    /// Opens a root directory.
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, FsError> {
        let root = root.into();
        if !root.is_dir() {
            return Err(FsError::BadRoot(root));
        }
        Ok(RootFs { root })
    }

    /// The root directory on the host.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Maps an absolute path inside the root to the host path.
    pub fn host_path(&self, path: impl AsRef<Path>) -> Result<PathBuf, FsError> {
        let path = path.as_ref();
        if !path.is_absolute() {
            return Err(FsError::NotAbsolute(path.to_owned()));
        }
        let mut host = self.root.clone();
        for component in path.components() {
            match component {
                Component::RootDir | Component::CurDir => {}
                Component::Normal(part) => host.push(part),
                Component::ParentDir | Component::Prefix(_) => {
                    return Err(FsError::Forbidden(path.to_owned()));
                }
            }
        }
        Ok(host)
    }

    /// Reads a UTF-8 text file. Returns `Ok(None)` when the file does not
    /// exist (or a parent is not a directory). Invalid UTF-8 is replaced
    /// rather than rejected, because config files in the wild are not
    /// always clean.
    pub fn read_to_string(&self, path: impl AsRef<Path>) -> Result<Option<String>, FsError> {
        let path = path.as_ref();
        let host = self.host_path(path)?;
        match std::fs::read(&host) {
            Ok(bytes) => Ok(Some(String::from_utf8_lossy(&bytes).into_owned())),
            Err(e) if is_absent(&e) => Ok(None),
            Err(source) => Err(FsError::Io {
                path: path.to_owned(),
                source,
            }),
        }
    }

    /// Whether something exists at the path.
    pub fn exists(&self, path: impl AsRef<Path>) -> Result<bool, FsError> {
        let path = path.as_ref();
        let host = self.host_path(path)?;
        match std::fs::metadata(&host) {
            Ok(_) => Ok(true),
            Err(e) if is_absent(&e) => Ok(false),
            Err(source) => Err(FsError::Io {
                path: path.to_owned(),
                source,
            }),
        }
    }
}

fn is_absent(e: &io::Error) -> bool {
    matches!(
        e.kind(),
        io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/env-001/ubuntu-noble")
    }

    #[test]
    fn maps_absolute_paths_under_the_root() {
        let fs = RootFs::new(fixture_root()).unwrap();
        assert_eq!(
            fs.host_path("/etc/os-release").unwrap(),
            fixture_root().join("etc/os-release")
        );
        assert!(fs.read_to_string("/etc/os-release").unwrap().is_some());
        assert!(fs.exists("/etc").unwrap());
    }

    #[test]
    fn missing_files_are_none_not_errors() {
        let fs = RootFs::new(fixture_root()).unwrap();
        assert_eq!(fs.read_to_string("/etc/does-not-exist").unwrap(), None);
        assert_eq!(fs.read_to_string("/etc/os-release/child").unwrap(), None);
        assert!(!fs.exists("/nope").unwrap());
    }

    #[test]
    fn rejects_relative_and_escaping_paths() {
        let fs = RootFs::new(fixture_root()).unwrap();
        assert!(matches!(
            fs.read_to_string("etc/os-release"),
            Err(FsError::NotAbsolute(_))
        ));
        assert!(matches!(
            fs.read_to_string("/etc/../../outside"),
            Err(FsError::Forbidden(_))
        ));
    }

    #[test]
    fn root_must_be_a_directory() {
        assert!(matches!(
            RootFs::new(fixture_root().join("etc/os-release")),
            Err(FsError::BadRoot(_))
        ));
    }
}

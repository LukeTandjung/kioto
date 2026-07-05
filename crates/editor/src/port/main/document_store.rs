use std::fmt;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// Domain location of a document. Files are the only implemented backing
/// store today, but callers no longer pass raw filesystem paths through the
/// whole editor API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentLocation {
    Filesystem(PathBuf),
}

impl DocumentLocation {
    pub fn filesystem(path: impl Into<PathBuf>) -> Self {
        Self::Filesystem(path.into())
    }

    pub fn as_filesystem_path(&self) -> &Path {
        match self {
            Self::Filesystem(path) => path,
        }
    }
}

impl From<PathBuf> for DocumentLocation {
    fn from(path: PathBuf) -> Self {
        Self::filesystem(path)
    }
}

impl fmt::Display for DocumentLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Filesystem(path) => write!(formatter, "{}", path.display()),
        }
    }
}

/// Document persistence capability. Locations are expressed in editor/domain
/// terms so the application layer does not care where documents live.
pub trait DocumentStore {
    fn load(&self, location: &DocumentLocation) -> Result<String, StoreError>;
    fn save(&self, location: &DocumentLocation, text: &str) -> Result<(), StoreError>;
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("could not read {location}: {source}")]
    Load {
        location: String,
        source: std::io::Error,
    },
    #[error("could not write {location}: {source}")]
    Save {
        location: String,
        source: std::io::Error,
    },
}

use std::path::Path;

use thiserror::Error;

/// Document persistence capability. Locations are filesystem paths today;
/// the port keeps `app` unaware of *where* documents live so other stores
/// (remote, in-memory) can slot in.
pub trait DocumentStore {
    fn load(&self, location: &Path) -> Result<String, StoreError>;
    fn save(&self, location: &Path, text: &str) -> Result<(), StoreError>;
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

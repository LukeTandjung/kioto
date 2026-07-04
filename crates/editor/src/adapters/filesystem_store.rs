use std::path::Path;

use crate::port::document_store::{DocumentStore, StoreError};

/// Filesystem-backed persistence: locations are paths, documents are files.
pub struct FilesystemStore;

impl DocumentStore for FilesystemStore {
    fn load(&self, location: &Path) -> Result<String, StoreError> {
        std::fs::read_to_string(location).map_err(|source| StoreError::Load {
            location: location.display().to_string(),
            source,
        })
    }

    fn save(&self, location: &Path, text: &str) -> Result<(), StoreError> {
        std::fs::write(location, text).map_err(|source| StoreError::Save {
            location: location.display().to_string(),
            source,
        })
    }
}

#[cfg(test)]
#[path = "filesystem_store.test.rs"]
mod tests;

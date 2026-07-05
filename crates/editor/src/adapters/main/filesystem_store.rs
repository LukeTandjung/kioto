use crate::port::document_store::{DocumentLocation, DocumentStore, StoreError};

/// Filesystem-backed persistence: filesystem locations are read/written as
/// plain text files.
pub struct FilesystemStore;

impl DocumentStore for FilesystemStore {
    fn load(&self, location: &DocumentLocation) -> Result<String, StoreError> {
        let path = location.as_filesystem_path();
        std::fs::read_to_string(path).map_err(|source| StoreError::Load {
            location: location.to_string(),
            source,
        })
    }

    fn save(&self, location: &DocumentLocation, text: &str) -> Result<(), StoreError> {
        let path = location.as_filesystem_path();
        std::fs::write(path, text).map_err(|source| StoreError::Save {
            location: location.to_string(),
            source,
        })
    }
}

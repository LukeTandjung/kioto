use crate::adapters::filesystem_store::FilesystemStore;
use crate::port::document_store::{DocumentLocation, DocumentStore, StoreError};

#[test]
fn saves_and_loads_round_trip() {
    let directory = std::env::temp_dir().join("kioto-filesystem-store-test");
    std::fs::create_dir_all(&directory).expect("temp dir");
    let path = directory.join("note.typ");
    let location = DocumentLocation::filesystem(path.clone());

    let store = FilesystemStore;
    store.save(&location, "= Saved\n").expect("save");
    assert_eq!(store.load(&location).expect("load"), "= Saved\n");

    std::fs::remove_file(&path).ok();
}

#[test]
fn loading_a_missing_file_is_a_typed_error() {
    let store = FilesystemStore;
    let missing = DocumentLocation::filesystem(
        std::env::temp_dir().join("kioto-filesystem-store-test/missing.typ"),
    );
    assert!(matches!(store.load(&missing), Err(StoreError::Load { .. })));
}

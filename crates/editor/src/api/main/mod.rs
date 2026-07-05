pub mod editor;

pub use editor::{EditorConfig, create_editor, open_editor};

pub use crate::app::render::EditorView;
pub use crate::app::style::EditorStyle;
pub use crate::port::document_store::{DocumentLocation, StoreError};

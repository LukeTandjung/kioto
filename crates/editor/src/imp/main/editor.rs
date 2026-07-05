use gpui::{App, AppContext as _, Entity, Window};

use crate::adapters::filesystem_store::FilesystemStore;
use crate::adapters::gpui_clipboard::GpuiClipboard;
use crate::api::editor::EditorConfig;
use crate::app::documents::typst::TypstDocument;
use crate::app::editor::Editor;
use crate::app::render::EditorView;
use crate::core::document::EditorDocument;
use crate::port::document_store::{DocumentLocation, DocumentStore, StoreError};

/// Composition root: chooses the concrete document model and constructs
/// adapters for the ports the GPUI editor component depends on.
pub fn create_editor(
    config: EditorConfig,
    window: &mut Window,
    cx: &mut App,
) -> Entity<EditorView> {
    let store: Box<dyn DocumentStore> = Box::new(FilesystemStore);
    build_editor_view(config, store, window, cx)
}

pub fn open_editor(
    location: DocumentLocation,
    mut config: EditorConfig,
    window: &mut Window,
    cx: &mut App,
) -> Result<Entity<EditorView>, StoreError> {
    let store: Box<dyn DocumentStore> = Box::new(FilesystemStore);
    config.text = store.load(&location)?;
    config.location = Some(location);
    Ok(build_editor_view(config, store, window, cx))
}

fn build_editor_view(
    config: EditorConfig,
    store: Box<dyn DocumentStore>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<EditorView> {
    let document: Box<dyn EditorDocument> = Box::new(TypstDocument::new(config.text));
    let editor = Editor::new(document);
    cx.new(|cx| {
        EditorView::new(
            editor,
            config.style,
            config.title,
            config.location,
            store,
            |app| Box::new(GpuiClipboard::new(app)),
            window,
            cx,
        )
    })
}

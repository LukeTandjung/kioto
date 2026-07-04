use gpui::{App, AppContext as _, Entity, Window};

use crate::adapters::filesystem_store::FilesystemStore;
use crate::api::editor::EditorConfig;
use crate::app::documents::typst::TypstDocument;
use crate::app::editor::Editor;
use crate::app::render::EditorView;
use crate::port::document_store::DocumentStore;

/// Composition root: chooses the concrete document model and constructs
/// adapters for the ports `app` depends on.
pub fn create_editor(
    config: EditorConfig,
    window: &mut Window,
    cx: &mut App,
) -> Entity<EditorView> {
    let store: Box<dyn DocumentStore> = Box::new(FilesystemStore);
    let text = match (&config.location, config.text.is_empty()) {
        (Some(location), true) => store.load(location).unwrap_or_default(),
        _ => config.text,
    };
    let editor = Editor::new(TypstDocument::new(text));
    cx.new(|cx| {
        EditorView::new(
            editor,
            config.style,
            config.title,
            config.location,
            store,
            window,
            cx,
        )
    })
}

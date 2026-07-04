use gpui::{App, AppContext as _, Entity, Window};

use crate::api::editor::EditorConfig;
use crate::app::documents::typst::TypstDocument;
use crate::app::editor::Editor;
use crate::app::render::EditorView;

/// Composition root: chooses the concrete document model and (from
/// milestone 5) constructs adapters for the ports `app` depends on.
pub fn create_editor(
    config: EditorConfig,
    window: &mut Window,
    cx: &mut App,
) -> Entity<EditorView> {
    let editor = Editor::new(TypstDocument::new(config.text));
    cx.new(|cx| EditorView::new(editor, config.style, config.title, window, cx))
}

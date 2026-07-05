use gpui::{App, Entity, Window};

use crate::app::render::EditorView;
use crate::app::style::EditorStyle;
use crate::port::document_store::{DocumentLocation, StoreError};

/// Everything needed to construct an editor. A plain value object; the
/// style is an input, not a port.
pub struct EditorConfig {
    pub text: String,
    pub title: String,
    pub style: EditorStyle,
    /// Where the document persists. Construction never loads implicitly;
    /// callers provide `text`, and `ctrl-s` saves back to this location.
    pub location: Option<DocumentLocation>,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            text: String::new(),
            title: "scratch".into(),
            style: EditorStyle::default(),
            location: None,
        }
    }
}

/// Creates a Typst editor view from already-loaded text.
pub fn create_editor(
    config: EditorConfig,
    window: &mut Window,
    cx: &mut App,
) -> Entity<EditorView> {
    crate::imp::create_editor(config, window, cx)
}

/// Loads a Typst document explicitly, then creates an editor view that saves
/// back to `location`.
pub fn open_editor(
    location: DocumentLocation,
    config: EditorConfig,
    window: &mut Window,
    cx: &mut App,
) -> Result<Entity<EditorView>, StoreError> {
    crate::imp::open_editor(location, config, window, cx)
}

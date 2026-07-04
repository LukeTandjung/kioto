use gpui::{App, Entity, Window};

use crate::app::render::EditorView;
use crate::app::style::EditorStyle;

/// Everything needed to construct an editor. A plain value object; the
/// style is an input, not a port.
pub struct EditorConfig {
    pub text: String,
    pub title: String,
    pub style: EditorStyle,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            text: String::new(),
            title: "scratch".into(),
            style: EditorStyle::default(),
        }
    }
}

/// Creates a Typst editor view ready to place in a GPUI window.
pub fn create_editor(
    config: EditorConfig,
    window: &mut Window,
    cx: &mut App,
) -> Entity<EditorView> {
    crate::imp::create_editor(config, window, cx)
}

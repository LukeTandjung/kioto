use gpui::{Hsla, Rgba, rgb, rgba};

use crate::core::mode::EditorMode;

/// Design tokens from the "Helix Editor" Claude Design project
/// (issues/build-editor-view.md, "Visual design requirements"). A plain
/// value object passed into the editor at construction.
pub struct EditorStyle {
    pub background: Rgba,
    pub status_background: Rgba,
    pub status_border: Rgba,
    pub accent: Rgba,
    pub text: Rgba,
    pub text_muted: Rgba,
    pub gutter_number: Rgba,
    pub gutter_number_current: Rgba,
    pub current_line: Rgba,
}

impl Default for EditorStyle {
    fn default() -> Self {
        Self {
            background: rgb(0x0E0E0E),
            status_background: rgb(0x1A1A1A),
            status_border: rgba(0xFFFFFF0D),
            accent: rgb(0xE8B84A),
            text: rgba(0xFFFFFFD1),
            text_muted: rgba(0xFFFFFF80),
            gutter_number: rgba(0xFFFFFF3D),
            gutter_number_current: rgb(0xFFFFFF),
            current_line: rgba(0xFFFFFF0A),
        }
    }
}

impl EditorStyle {
    /// Monochrome selection overlay; brighter in Visual mode, dimmer while
    /// typing.
    pub fn selection(&self, mode: &EditorMode) -> Hsla {
        let alpha = if mode.is_insert() {
            0.12
        } else if mode.is_visual() {
            0.26
        } else {
            0.18
        };
        gpui::white().opacity(alpha)
    }

    /// Mode segment colors for the status line: `(foreground, background)`.
    /// INSERT inverts to solid white — the "live typing" signal.
    pub fn mode_segment(&self, mode: &EditorMode) -> (Rgba, Rgba) {
        if mode.is_insert() {
            (self.background, rgb(0xFFFFFF))
        } else if mode.is_visual() {
            (rgb(0xFFFFFF), rgba(0xFFFFFF38))
        } else {
            (rgb(0xFFFFFF), rgba(0xFFFFFF14))
        }
    }
}

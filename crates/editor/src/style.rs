use gpui::{Hsla, Rgba, rgb, rgba};

use crate::editor::EditorMode;

/// Design tokens from the "Helix Editor" Claude Design project
/// (issues/build-editor-view.md, "Visual design requirements").
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
    /// Monochrome selection overlay; brighter in Select mode, dimmer while
    /// typing. (The design hides Insert-mode selections entirely, but an
    /// invisible shift/mouse selection would be unusable, so we dim instead.)
    pub fn selection(&self, mode: EditorMode) -> Hsla {
        let alpha = match mode {
            EditorMode::Insert => 0.12,
            EditorMode::Normal => 0.18,
            EditorMode::Select => 0.26,
        };
        gpui::white().opacity(alpha)
    }

    /// Mode segment colors for the status line: `(foreground, background)`.
    /// INSERT inverts to solid white — the "live typing" signal.
    pub fn mode_segment(&self, mode: EditorMode) -> (Rgba, Rgba) {
        match mode {
            EditorMode::Insert => (self.background, rgb(0xFFFFFF)),
            EditorMode::Normal => (rgb(0xFFFFFF), rgba(0xFFFFFF14)),
            EditorMode::Select => (rgb(0xFFFFFF), rgba(0xFFFFFF38)),
        }
    }
}

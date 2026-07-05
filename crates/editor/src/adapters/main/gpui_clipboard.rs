use gpui::{App, ClipboardItem};

use crate::port::clipboard::Clipboard;

/// GPUI-backed system clipboard. GPUI's clipboard lives on `App`, so this
/// adapter borrows the app for the duration of one interaction — the view
/// constructs it fresh inside each event handler.
pub struct GpuiClipboard<'a> {
    app: &'a mut App,
}

impl<'a> GpuiClipboard<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
}

impl Clipboard for GpuiClipboard<'_> {
    fn read(&mut self) -> Option<String> {
        self.app.read_from_clipboard()?.text()
    }

    fn write(&mut self, text: String) {
        self.app.write_to_clipboard(ClipboardItem::new_string(text));
    }
}

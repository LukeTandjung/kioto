use crate::port::clipboard::Clipboard;

/// In-memory clipboard for tests and headless use.
#[derive(Default)]
pub struct MemoryClipboard {
    contents: Option<String>,
}

impl Clipboard for MemoryClipboard {
    fn read(&mut self) -> Option<String> {
        self.contents.clone()
    }

    fn write(&mut self, text: String) {
        self.contents = Some(text);
    }
}

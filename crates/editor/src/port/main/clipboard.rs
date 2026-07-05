/// System clipboard capability. Yank/paste in the modes stays pure by
/// returning actions as data; actually touching the clipboard is a side
/// effect, so it crosses this port.
pub trait Clipboard {
    /// The clipboard's current text, if it holds any.
    fn read(&mut self) -> Option<String>;
    fn write(&mut self, text: String);
}

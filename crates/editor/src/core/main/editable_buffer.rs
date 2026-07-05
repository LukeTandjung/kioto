use crate::core::position::Range;

/// The language-neutral buffer contract: text reads and primitive edits,
/// nothing else. Each supported language provides its own implementation
/// (e.g. `TypstDocument` wrapping `typst_syntax::Source`); the editor holds
/// this abstraction, never a concrete language type.
///
/// Contract: out-of-range and mid-character positions are errors defined out
/// of existence — implementations use `Range::byte_range_in` to clamp to the
/// buffer length and to the previous UTF-8 character boundary.
pub trait EditableBuffer {
    fn text(&self) -> &str;

    /// The single write path: replaces `range` with `new_text`.
    fn replace(&mut self, range: Range, new_text: &str);
}

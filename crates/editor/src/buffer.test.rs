use super::TextBuffer;

#[test]
fn utf16_ranges_round_trip_through_emoji() {
    let buffer = TextBuffer::new("a💝b");
    let emoji_end = "a💝".len();
    assert_eq!(buffer.range_to_utf16(&(0..emoji_end)), 0..3);
    assert_eq!(buffer.range_from_utf16(&(0..3)), 0..emoji_end);
}

#[test]
fn line_access_keeps_trailing_empty_line() {
    let buffer = TextBuffer::new("a\nb\n");
    assert_eq!(buffer.line_count(), 3);
    assert_eq!(buffer.line(2).text, "");
}

#[test]
fn line_range_excludes_line_breaks() {
    let buffer = TextBuffer::new("ab\r\ncd\ne");
    assert_eq!(buffer.line(0).range, 0..2);
    assert_eq!(buffer.line(1).range, 4..6);
    assert_eq!(buffer.line(2).range, 7..8);
}

#[test]
fn line_clamps_row_to_last_line() {
    let buffer = TextBuffer::new("ab\ncd");
    assert_eq!(buffer.line(99).range, 3..5);
}

#[test]
fn grapheme_boundaries_cross_lines() {
    let buffer = TextBuffer::new("a\nb");
    assert_eq!(buffer.next_grapheme_boundary(1), 2);
    assert_eq!(buffer.previous_grapheme_boundary(2), 1);

    let crlf = TextBuffer::new("a\r\nb");
    assert_eq!(crlf.next_grapheme_boundary(1), 3);
    assert_eq!(crlf.previous_grapheme_boundary(3), 1);
}

#[test]
fn grapheme_boundaries_handle_combining_marks() {
    let buffer = TextBuffer::new("e\u{301}x");
    assert_eq!(buffer.next_grapheme_boundary(0), 3);
    assert_eq!(buffer.previous_grapheme_boundary(3), 0);
}

#[test]
fn grapheme_boundaries_saturate_at_buffer_edges() {
    let buffer = TextBuffer::new("ab");
    assert_eq!(buffer.previous_grapheme_boundary(0), 0);
    assert_eq!(buffer.next_grapheme_boundary(2), 2);
}

#[test]
fn replace_clips_to_char_boundaries() {
    // A start offset inside 💝 (bytes 1..5) clips down to its boundary.
    let mut buffer = TextBuffer::new("a💝b");
    buffer.replace(2..5, "");
    assert_eq!(buffer.text(), "ab");

    // A range entirely inside one char collapses to a no-op.
    let mut buffer = TextBuffer::new("a💝b");
    buffer.replace(2..3, "");
    assert_eq!(buffer.text(), "a💝b");
}

#[test]
fn replace_inserts_and_deletes() {
    let mut buffer = TextBuffer::new("hello world");
    buffer.replace(5..5, ",");
    assert_eq!(buffer.text(), "hello, world");
    buffer.replace(0..5, "goodbye");
    assert_eq!(buffer.text(), "goodbye, world");
}

#[test]
fn offset_point_round_trip() {
    let buffer = TextBuffer::new("ab\ncdef\n");
    assert_eq!(buffer.offset_to_point(5), (1, 2));
    assert_eq!(buffer.point_to_offset(1, 2), 5);
    assert_eq!(buffer.point_to_offset(1, 99), 7);
    assert_eq!(buffer.offset_to_point(buffer.len()), (2, 0));
}

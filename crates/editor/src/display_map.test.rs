use super::{DisplayPoint, DisplaySnapshot, WrapMap};
use crate::buffer::TextBuffer;

/// Wraps every `width` bytes, on char boundaries — a stand-in for pixel
/// measurement in tests.
fn wrap_every(width: usize) -> impl FnMut(&str) -> Vec<usize> {
    move |text: &str| {
        text.char_indices()
            .map(|(index, _)| index)
            .filter(|index| *index > 0 && index % width == 0)
            .collect()
    }
}

#[test]
fn unwrapped_map_is_identity() {
    let buffer = TextBuffer::new("ab\ncdef\n");
    let map = WrapMap::unwrapped(&buffer);
    let snapshot = DisplaySnapshot::new(&buffer, &map);

    assert_eq!(snapshot.row_count(), 3);
    assert_eq!(snapshot.row_text(1), "cdef");

    let point = snapshot.offset_to_display(5);
    assert_eq!(point, DisplayPoint { row: 1, column: 2 });
    assert_eq!(snapshot.display_to_offset(point), 5);
}

#[test]
fn wrapped_lines_span_multiple_display_rows() {
    let buffer = TextBuffer::new("abcdef\ngh");
    let map = WrapMap::build(&buffer, 100., wrap_every(3));
    let snapshot = DisplaySnapshot::new(&buffer, &map);

    assert_eq!(snapshot.row_count(), 3);
    assert_eq!(snapshot.row_text(0), "abc");
    assert_eq!(snapshot.row_text(1), "def");
    assert_eq!(snapshot.row_text(2), "gh");
    assert!(snapshot.row(0).is_line_start);
    assert!(!snapshot.row(1).is_line_start);
    assert_eq!(snapshot.row(1).buffer_row, 0);
}

#[test]
fn offsets_round_trip_across_wrap_boundaries() {
    let buffer = TextBuffer::new("abcdef");
    let map = WrapMap::build(&buffer, 100., wrap_every(3));
    let snapshot = DisplaySnapshot::new(&buffer, &map);

    // Offset 4 is "e", the second byte of the second display row.
    let point = snapshot.offset_to_display(4);
    assert_eq!(point, DisplayPoint { row: 1, column: 1 });
    assert_eq!(snapshot.display_to_offset(point), 4);
}

#[test]
fn display_points_clamp_to_content() {
    let buffer = TextBuffer::new("ab");
    let map = WrapMap::unwrapped(&buffer);
    let snapshot = DisplaySnapshot::new(&buffer, &map);

    let clamped = snapshot.display_to_offset(DisplayPoint { row: 9, column: 9 });
    assert_eq!(clamped, 2);
}

#[test]
fn cache_invalidates_on_edit_and_resize() {
    let mut buffer = TextBuffer::new("ab");
    let map = WrapMap::build(&buffer, 100., wrap_every(3));
    assert!(map.is_valid_for(&buffer, 100.));
    assert!(!map.is_valid_for(&buffer, 200.));

    buffer.replace(0..0, "x");
    assert!(!map.is_valid_for(&buffer, 100.));
}

#[test]
fn empty_lines_keep_one_display_row() {
    let buffer = TextBuffer::new("a\n\nb");
    let map = WrapMap::build(&buffer, 100., wrap_every(1));
    let snapshot = DisplaySnapshot::new(&buffer, &map);

    assert_eq!(snapshot.row_count(), 3);
    assert_eq!(snapshot.row_text(1), "");
}

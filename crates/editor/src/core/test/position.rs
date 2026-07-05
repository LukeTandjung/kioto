use crate::core::position::{Position, Range};

#[test]
fn range_normalizes_endpoint_order() {
    let range = Range::new(Position(5), Position(2));
    assert_eq!(range.start, Position(2));
    assert_eq!(range.end, Position(5));
    assert_eq!(range.len(), 3);
}

#[test]
fn caret_is_empty() {
    assert!(Range::caret(Position(3)).is_empty());
    assert!(!Range::new(Position(0), Position(1)).is_empty());
}

#[test]
fn byte_range_in_clamps_to_utf8_boundaries() {
    let text = "a💝b";
    assert_eq!(
        Range::new(Position(2), Position(99)).byte_range_in(text),
        1..6
    );
    assert_eq!(
        Range::new(Position(2), Position(3)).byte_range_in(text),
        1..1
    );
}

#[test]
fn slice_uses_clamped_source_range() {
    let text = "a💝b";
    assert_eq!(Range::new(Position(2), Position(99)).slice(text), "💝b");
}

#[test]
fn utf16_conversion_round_trips_source_ranges() {
    let text = "a💝b";
    let source = Range::new(Position(1), Position(5));
    let utf16 = source.to_utf16(text);
    assert_eq!(utf16, 1..3);
    assert_eq!(Range::from_utf16(text, utf16), source);
}

#[test]
fn utf16_offsets_inside_surrogate_pair_floor_to_source_boundary() {
    let text = "a💝b";
    assert_eq!(Range::from_utf16(text, 2..2), Range::caret(Position(1)));
}

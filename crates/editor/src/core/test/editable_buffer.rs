use crate::core::position::{Position, Range};

#[test]
fn clamps_past_end_of_text() {
    let clamped = Range::new(Position(1), Position(99)).byte_range_in("ab");
    assert_eq!(clamped, 1..2);
}

#[test]
fn clamps_mid_character_offsets_to_previous_boundary() {
    // 💝 occupies bytes 1..5.
    let clamped = Range::new(Position(2), Position(5)).byte_range_in("a💝b");
    assert_eq!(clamped, 1..5);
}

#[test]
fn collapses_inverted_clamps_to_empty() {
    // Both endpoints inside 💝 floor to 1.
    let clamped = Range::new(Position(2), Position(3)).byte_range_in("a💝b");
    assert_eq!(clamped, 1..1);
}

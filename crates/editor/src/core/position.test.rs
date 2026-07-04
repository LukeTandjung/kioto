use super::{Position, Range};

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

use crate::core::position::Position;
use crate::core::selection::Selection;

#[test]
fn set_none_means_file_start_and_end() {
    let mut selection = Selection::new(Position(3), Position(7));
    selection.set(None, None);
    assert_eq!(selection.from_position, Position(0));
    assert_eq!(selection.to_position, Position(usize::MAX));
}

#[test]
fn backwards_selections_are_preserved() {
    let selection = Selection::new(Position(7), Position(3));
    assert_eq!(selection.from_position, Position(7));
    assert_eq!(selection.to_position, Position(3));
}

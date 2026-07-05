use crate::core::motion::{Motion, line_ranges, resolve};
use crate::core::position::Position;

fn run(text: &str, at: usize, motion: Motion) -> usize {
    resolve(text, Position(at), motion).0
}

#[test]
fn horizontal_steps_are_grapheme_aware() {
    let text = "a💝b";
    assert_eq!(run(text, 0, Motion::Right), 1);
    assert_eq!(run(text, 1, Motion::Right), 5);
    assert_eq!(run(text, 5, Motion::Left), 1);
    assert_eq!(run(text, 0, Motion::Left), 0);
    assert_eq!(run(text, 6, Motion::Right), 6);
}

#[test]
fn vertical_steps_preserve_column_and_clamp() {
    let text = "abcdef\nab\nabcd";
    assert_eq!(run(text, 4, Motion::Down), 9); // col 4 clamps to "ab" end
    assert_eq!(run(text, 9, Motion::Down), 12); // col restarts from line start
    assert_eq!(run(text, 12, Motion::Up), 9);
    assert_eq!(run(text, 2, Motion::Up), 2); // first line stays put
}

#[test]
fn line_start_and_end() {
    let text = "ab\ncdef";
    assert_eq!(run(text, 5, Motion::LineStart), 3);
    assert_eq!(run(text, 5, Motion::LineEnd), 7);
    assert_eq!(run(text, 1, Motion::LineEnd), 2);
}

#[test]
fn document_start_and_end() {
    let text = "ab\ncd";
    assert_eq!(run(text, 3, Motion::DocumentStart), 0);
    assert_eq!(run(text, 0, Motion::DocumentEnd), 5);
}

#[test]
fn word_motions() {
    //          0123456789012345
    let text = "one two,  three";
    assert_eq!(run(text, 0, Motion::WordForward), 4); // one -> two
    assert_eq!(run(text, 4, Motion::WordForward), 7); // two -> ,
    assert_eq!(run(text, 7, Motion::WordForward), 10); // , -> three
    assert_eq!(run(text, 10, Motion::WordBack), 7);
    assert_eq!(run(text, 5, Motion::WordBack), 4); // mid-word -> word start
    assert_eq!(run(text, 0, Motion::WordEnd), 2); // last char of "one"
    assert_eq!(run(text, 2, Motion::WordEnd), 6); // then last char of "two"
    assert_eq!(run(text, 14, Motion::WordForward), 15); // end of text
}

#[test]
fn line_ranges_handle_crlf_and_trailing_newline() {
    assert_eq!(line_ranges("ab\r\ncd\n"), vec![0..2, 4..6, 7..7]);
    assert_eq!(line_ranges(""), vec![0..0]);
}

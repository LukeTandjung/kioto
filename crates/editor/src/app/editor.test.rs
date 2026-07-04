use super::Editor;
use crate::app::documents::typst::TypstDocument;
use crate::core::editable_buffer::EditableBuffer;
use crate::core::mode::Input;
use crate::core::position::{Position, Range};

fn editor(text: &str) -> Editor<TypstDocument> {
    Editor::new(TypstDocument::new(text))
}

fn type_keys(editor: &mut Editor<TypstDocument>, keys: &str) {
    for key in keys.chars() {
        editor.handle_input(Input::Char(key));
    }
}

#[test]
fn starts_in_normal_mode_and_navigates() {
    let mut editor = editor("one two\nthree");
    type_keys(&mut editor, "w");
    assert_eq!(editor.cursor(), Position(4));
    type_keys(&mut editor, "j0");
    assert_eq!(editor.cursor(), Position(8));
    type_keys(&mut editor, "$");
    assert_eq!(editor.cursor(), Position(13));
    type_keys(&mut editor, "gg");
    assert_eq!(editor.cursor(), Position(0));
}

#[test]
fn insert_mode_types_and_escapes() {
    let mut editor = editor("world");
    type_keys(&mut editor, "ihello ");
    assert_eq!(editor.document.text(), "hello world");
    editor.handle_input(Input::Escape);
    assert!(!editor.mode.is_insert());
    assert_eq!(editor.cursor(), Position(6));
}

#[test]
fn append_enters_insert_after_cursor() {
    let mut editor = editor("ab");
    type_keys(&mut editor, "a!");
    assert_eq!(editor.document.text(), "a!b");
}

#[test]
fn backspace_is_grapheme_aware() {
    let mut editor = editor("");
    type_keys(&mut editor, "ia💝");
    editor.handle_input(Input::Backspace);
    assert_eq!(editor.document.text(), "a");
}

#[test]
fn visual_selection_extends_and_deletes() {
    let mut editor = editor("one two three");
    type_keys(&mut editor, "vww");
    assert_eq!(
        editor.selected_range(),
        Some(Range::new(Position(0), Position(8)))
    );
    type_keys(&mut editor, "d");
    assert_eq!(editor.document.text(), "three");
    assert!(!editor.mode.is_visual());
    assert_eq!(editor.cursor(), Position(0));
}

#[test]
fn select_line_selects_through_the_line_break() {
    let mut editor = editor("one\ntwo\nthree");
    type_keys(&mut editor, "jxd");
    assert_eq!(editor.document.text(), "one\nthree");
}

#[test]
fn delete_without_selection_removes_char_under_cursor() {
    let mut editor = editor("abc");
    type_keys(&mut editor, "ld");
    assert_eq!(editor.document.text(), "ac");
}

#[test]
fn escape_from_visual_clears_selection() {
    let mut editor = editor("abc");
    type_keys(&mut editor, "vl");
    assert!(editor.selected_range().is_some());
    editor.handle_input(Input::Escape);
    assert!(editor.selected_range().is_none());
}

#[test]
fn ime_committed_text_replaces_selection() {
    let mut editor = editor("one two");
    type_keys(&mut editor, "vw");
    editor.insert_text("1 ");
    assert_eq!(editor.document.text(), "1 two");
}

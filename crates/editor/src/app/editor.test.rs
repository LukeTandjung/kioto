use super::Editor;
use crate::adapters::memory_clipboard::MemoryClipboard;
use crate::app::documents::typst::TypstDocument;
use crate::core::editable_buffer::EditableBuffer;
use crate::core::mode::{EditorAction, Input};
use crate::core::position::{Position, Range};
use crate::port::clipboard::Clipboard as _;

fn editor(text: &str) -> Editor<TypstDocument> {
    Editor::new(TypstDocument::new(text))
}

fn type_keys(editor: &mut Editor<TypstDocument>, keys: &str) {
    let mut clipboard = MemoryClipboard::default();
    for key in keys.chars() {
        editor.handle_input(Input::Char(key), &mut clipboard);
    }
}

fn press(editor: &mut Editor<TypstDocument>, input: Input) -> EditorAction {
    editor.handle_input(input, &mut MemoryClipboard::default())
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
    press(&mut editor, Input::Escape);
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
    press(&mut editor, Input::Backspace);
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
    press(&mut editor, Input::Escape);
    assert!(editor.selected_range().is_none());
}

#[test]
fn ime_committed_text_replaces_selection() {
    let mut editor = editor("one two");
    type_keys(&mut editor, "vw");
    editor.insert_text("1 ");
    assert_eq!(editor.document.text(), "1 two");
}

// --- Milestone 5: yank/paste, undo, search ---------------------------------

#[test]
fn visual_yank_writes_the_clipboard_and_returns_to_normal() {
    let mut editor = editor("one two");
    let mut clipboard = MemoryClipboard::default();
    editor.handle_input(Input::Char('v'), &mut clipboard);
    editor.handle_input(Input::Char('w'), &mut clipboard);
    editor.handle_input(Input::Char('y'), &mut clipboard);

    assert_eq!(clipboard.read().as_deref(), Some("one "));
    assert!(!editor.mode.is_visual());
    assert_eq!(editor.document.text(), "one two");
    assert_eq!(editor.cursor(), Position(0));
}

#[test]
fn paste_inserts_after_the_cursor() {
    let mut editor = editor("ab");
    let mut clipboard = MemoryClipboard::default();
    clipboard.write("XY".into());
    editor.handle_input(Input::Char('p'), &mut clipboard);

    assert_eq!(editor.document.text(), "aXYb");
    assert_eq!(editor.cursor(), Position(3));
}

#[test]
fn paste_replaces_the_selection() {
    // Selections in normal mode come from the mouse; visual mode has no
    // `p` in the keybind table, so exercise the action directly.
    let mut editor = editor("one two");
    let mut clipboard = MemoryClipboard::default();
    clipboard.write("1".into());
    editor.selections = vec![crate::core::selection::Selection::new(
        Position(0),
        Position(4),
    )];
    editor.apply(EditorAction::Paste, &mut clipboard);

    assert_eq!(editor.document.text(), "1two");
    assert_eq!(editor.cursor(), Position(1));
}

#[test]
fn undo_reverses_a_typed_run_as_one_transaction() {
    let mut editor = editor("world");
    type_keys(&mut editor, "ihello ");
    press(&mut editor, Input::Escape);
    assert_eq!(editor.document.text(), "hello world");

    type_keys(&mut editor, "u");
    assert_eq!(editor.document.text(), "world");
    assert_eq!(editor.cursor(), Position(0));
}

#[test]
fn undo_restores_a_deleted_selection() {
    let mut editor = editor("one two three");
    type_keys(&mut editor, "vwwd");
    assert_eq!(editor.document.text(), "three");

    type_keys(&mut editor, "u");
    assert_eq!(editor.document.text(), "one two three");
}

#[test]
fn undo_with_nothing_to_undo_is_a_no_op() {
    let mut editor = editor("abc");
    type_keys(&mut editor, "u");
    assert_eq!(editor.document.text(), "abc");
}

#[test]
fn search_jumps_and_repeats_with_wrapping() {
    let mut editor = editor("alpha beta alpha beta");
    editor.search("beta");
    assert_eq!(editor.cursor(), Position(6));

    type_keys(&mut editor, "n");
    assert_eq!(editor.cursor(), Position(17));
    // Wraps back to the first match.
    type_keys(&mut editor, "n");
    assert_eq!(editor.cursor(), Position(6));
    // And backwards, wrapping to the last.
    type_keys(&mut editor, "NN");
    assert_eq!(editor.cursor(), Position(6));
}

#[test]
fn search_with_no_match_stays_put() {
    let mut editor = editor("alpha");
    editor.search("zeta");
    assert_eq!(editor.cursor(), Position(0));
    assert_eq!(editor.search_query(), Some("zeta"));
}

#[test]
fn start_search_action_is_surfaced_to_the_caller() {
    let mut editor = editor("abc");
    assert_eq!(
        press(&mut editor, Input::Char('/')),
        EditorAction::StartSearch
    );
}

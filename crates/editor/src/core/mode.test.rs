use super::{EditorAction, EditorMode, Input};
use crate::core::motion::Motion;

#[test]
fn normal_mode_dispatches_the_keybind_table() {
    let mut mode = EditorMode::normal();
    assert_eq!(
        mode.handle_input(Input::Char('h')),
        EditorAction::Move(Motion::Left)
    );
    assert_eq!(
        mode.handle_input(Input::Char('w')),
        EditorAction::Move(Motion::WordForward)
    );
    assert_eq!(
        mode.handle_input(Input::Char('i')),
        EditorAction::EnterInsert {
            after_cursor: false
        }
    );
    assert_eq!(
        mode.handle_input(Input::Char('a')),
        EditorAction::EnterInsert { after_cursor: true }
    );
    assert_eq!(
        mode.handle_input(Input::Char('v')),
        EditorAction::EnterVisual
    );
    assert_eq!(
        mode.handle_input(Input::Char('x')),
        EditorAction::SelectLine
    );
    assert_eq!(
        mode.handle_input(Input::Char('d')),
        EditorAction::DeleteSelection
    );
    assert_eq!(mode.handle_input(Input::Char('u')), EditorAction::Undo);
    assert_eq!(
        mode.handle_input(Input::Char('/')),
        EditorAction::StartSearch
    );
}

#[test]
fn g_g_is_a_two_key_sequence() {
    let mut mode = EditorMode::normal();
    assert_eq!(mode.handle_input(Input::Char('g')), EditorAction::None);
    assert_eq!(
        mode.handle_input(Input::Char('g')),
        EditorAction::Move(Motion::DocumentStart)
    );

    // A different second key cancels the pending sequence.
    assert_eq!(mode.handle_input(Input::Char('g')), EditorAction::None);
    assert_eq!(mode.handle_input(Input::Char('x')), EditorAction::None);
    assert_eq!(
        mode.handle_input(Input::Char('x')),
        EditorAction::SelectLine
    );
}

#[test]
fn insert_mode_commits_text_and_escapes() {
    let mut mode = EditorMode::insert();
    assert_eq!(
        mode.handle_input(Input::Char('q')),
        EditorAction::InsertText("q".into())
    );
    assert_eq!(
        mode.handle_input(Input::Enter),
        EditorAction::InsertText("\n".into())
    );
    assert_eq!(
        mode.handle_input(Input::Backspace),
        EditorAction::DeleteBackward
    );
    assert_eq!(mode.handle_input(Input::Escape), EditorAction::EnterNormal);
    // Modal keys mean nothing in insert mode.
    assert_eq!(
        mode.handle_input(Input::Char('h')),
        EditorAction::InsertText("h".into())
    );
    assert_eq!(
        mode.handle_input(Input::Left),
        EditorAction::Move(Motion::Left)
    );
}

#[test]
fn visual_mode_extends_and_exits() {
    let mut mode = EditorMode::visual();
    assert_eq!(
        mode.handle_input(Input::Char('l')),
        EditorAction::Extend(Motion::Right)
    );
    assert_eq!(
        mode.handle_input(Input::Char('e')),
        EditorAction::Extend(Motion::WordEnd)
    );
    assert_eq!(
        mode.handle_input(Input::Char('d')),
        EditorAction::DeleteSelection
    );
    assert_eq!(mode.handle_input(Input::Escape), EditorAction::EnterNormal);
    assert_eq!(
        mode.handle_input(Input::Char('v')),
        EditorAction::EnterNormal
    );
}

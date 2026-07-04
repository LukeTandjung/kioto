use super::{Edit, History};
use crate::core::position::Position;

fn insert(at: usize, text: &str) -> Edit {
    Edit {
        range: at..at,
        old_text: String::new(),
        new_text: text.into(),
        cursor_before: Position(at),
        cursor_after: Position(at + text.len()),
    }
}

fn backspace(range: std::ops::Range<usize>, old_text: &str) -> Edit {
    Edit {
        range: range.clone(),
        old_text: old_text.into(),
        new_text: String::new(),
        cursor_before: Position(range.end),
        cursor_after: Position(range.start),
    }
}

#[test]
fn contiguous_typing_groups_into_one_transaction() {
    let mut history = History::default();
    history.record(insert(0, "h"));
    history.record(insert(1, "i"));

    let transaction = history.undo().expect("one transaction");
    assert_eq!(transaction.len(), 2);
    assert!(history.undo().is_none());
}

#[test]
fn newline_starts_a_new_transaction() {
    let mut history = History::default();
    history.record(insert(0, "a"));
    history.record(insert(1, "\n"));

    assert_eq!(history.undo().expect("newline transaction").len(), 1);
    assert_eq!(history.undo().expect("typing transaction").len(), 1);
}

#[test]
fn non_contiguous_edits_do_not_group() {
    let mut history = History::default();
    history.record(insert(0, "a"));
    history.record(insert(5, "b"));

    assert_eq!(history.undo().expect("second transaction").len(), 1);
    assert_eq!(history.undo().expect("first transaction").len(), 1);
}

#[test]
fn contiguous_backspaces_group() {
    let mut history = History::default();
    history.record(backspace(2..3, "c"));
    history.record(backspace(1..2, "b"));

    assert_eq!(history.undo().expect("one transaction").len(), 2);
}

#[test]
fn break_group_splits_typing() {
    let mut history = History::default();
    history.record(insert(0, "a"));
    history.break_group();
    history.record(insert(1, "b"));

    assert_eq!(history.undo().expect("second transaction").len(), 1);
    assert_eq!(history.undo().expect("first transaction").len(), 1);
}

#[test]
fn recording_clears_redo() {
    let mut history = History::default();
    history.record(insert(0, "a"));
    history.undo();
    history.record(insert(0, "b"));

    assert!(history.redo().is_none());
}

#[test]
fn undo_redo_round_trip() {
    let mut history = History::default();
    history.record(insert(0, "a"));

    let undone = history.undo().expect("transaction");
    let redone = history.redo().expect("same transaction back");
    assert_eq!(undone.len(), redone.len());
    assert!(history.undo().is_some());
}

#[test]
fn no_op_edits_are_ignored() {
    let mut history = History::default();
    history.record(Edit {
        range: 0..1,
        old_text: "a".into(),
        new_text: "a".into(),
        cursor_before: Position(0),
        cursor_after: Position(0),
    });

    assert!(history.undo().is_none());
}

use std::ops::Range;
use std::time::{Duration, Instant};

use crate::selection::Selection;

/// One buffer mutation, recorded against the pre-edit buffer state.
#[derive(Clone, Debug)]
pub struct Edit {
    pub range: Range<usize>,
    pub old_text: String,
    pub new_text: String,
    pub selection_before: Selection,
    pub selection_after: Selection,
}

/// Undo/redo stacks with automatic transaction grouping: rapid contiguous
/// typing, contiguous backspacing, and IME re-replacement collapse into one
/// transaction, so a single undo removes a typed run rather than one
/// character.
#[derive(Default)]
pub struct History {
    undo_stack: Vec<Vec<Edit>>,
    redo_stack: Vec<Vec<Edit>>,
    last_recorded_at: Option<Instant>,
}

const GROUP_INTERVAL: Duration = Duration::from_millis(500);

impl History {
    /// Records an edit, grouping it into the previous transaction when it
    /// continues one within the grouping interval. Recording clears the redo
    /// stack.
    pub fn record(&mut self, edit: Edit) {
        if edit.old_text == edit.new_text {
            return;
        }
        self.redo_stack.clear();

        let now = Instant::now();
        let within_interval = self
            .last_recorded_at
            .is_some_and(|at| now.duration_since(at) <= GROUP_INTERVAL);
        self.last_recorded_at = Some(now);

        if within_interval
            && let Some(transaction) = self.undo_stack.last_mut()
            && transaction
                .last()
                .is_some_and(|previous| continues(previous, &edit))
        {
            transaction.push(edit);
            return;
        }

        self.undo_stack.push(vec![edit]);
    }

    /// Ends the current transaction group; the next recorded edit starts a
    /// new one. Call on mode changes, cursor motion, and mouse clicks.
    pub fn break_group(&mut self) {
        self.last_recorded_at = None;
    }

    /// Pops the newest transaction for the caller to reverse (apply its
    /// edits back-to-front), moving it onto the redo stack.
    pub fn undo(&mut self) -> Option<Vec<Edit>> {
        let transaction = self.undo_stack.pop()?;
        self.redo_stack.push(transaction.clone());
        self.break_group();
        Some(transaction)
    }

    /// Pops the newest undone transaction for the caller to re-apply
    /// front-to-back, moving it back onto the undo stack.
    pub fn redo(&mut self) -> Option<Vec<Edit>> {
        let transaction = self.redo_stack.pop()?;
        self.undo_stack.push(transaction.clone());
        self.break_group();
        Some(transaction)
    }
}

/// Whether `edit` continues the transaction that ended with `previous`.
fn continues(previous: &Edit, edit: &Edit) -> bool {
    // Newlines end a typing group so undo works line-by-line.
    if edit.new_text.contains('\n') {
        return false;
    }

    let typing = previous.old_text.is_empty()
        && edit.old_text.is_empty()
        && edit.range.start == previous.range.start + previous.new_text.len();
    let backspacing = previous.new_text.is_empty()
        && edit.new_text.is_empty()
        && edit.range.end == previous.range.start;
    // IME composition repeatedly replaces the same marked region.
    let re_replacing = edit.range.start == previous.range.start
        && edit.range.len() == previous.new_text.len();

    typing || backspacing || re_replacing
}

#[cfg(test)]
#[path = "history.test.rs"]
mod tests;

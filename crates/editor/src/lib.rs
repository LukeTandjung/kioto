//! Core editor functionality for Kioto.

mod actions;
mod buffer;
mod editor;
mod element;
mod input;
mod position_map;
mod selection;

pub use actions::init;
pub use editor::{Editor, EditorMode};
pub use selection::Selection;

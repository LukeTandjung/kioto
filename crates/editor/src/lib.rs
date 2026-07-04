//! Core editor functionality for Kioto.
//!
//! Hexagonal shape per `issues/editor-business-requirements.md`: `api`
//! (public surface), `imp` (composition root), `app` (use-case
//! orchestration, document models, render orchestration), `core` (pure
//! editor domain logic), `port` (external capabilities), `adapters`
//! (side-effecting port implementations).
//!
//! Note: the pre-rebuild spike modules (`buffer.rs`, `editor.rs`,
//! `element.rs`, `highlights.rs`, `history.rs`, `input.rs`,
//! `position_map.rs`, `selection.rs`, `style.rs`, `actions.rs`,
//! `display_map.rs`) remain on disk, unlinked, as porting material —
//! `history.rs` and `highlights.rs` in particular are earmarked for
//! milestones 5 and 3. Delete them once their ports land.

pub mod adapters;
pub mod api;
pub mod app;
pub mod core;
mod imp;
pub mod port;

pub use api::*;

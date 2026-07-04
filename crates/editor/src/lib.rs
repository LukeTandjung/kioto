//! Core editor functionality for Kioto.
//!
//! Hexagonal shape per `issues/editor-business-requirements.md`: `api`
//! (public surface), `imp` (composition root), `app` (use-case
//! orchestration, document models, render orchestration), `core` (pure
//! editor domain logic), `port` (external capabilities), `adapters`
//! (side-effecting port implementations).
//!
//! The pre-rebuild spike modules were deleted after milestone 5 harvested
//! the last of them (`history.rs` → `core/history.rs`); they live on in
//! git history.

pub mod adapters;
pub mod api;
pub mod app;
pub mod core;
mod imp;
pub mod port;

pub use api::*;

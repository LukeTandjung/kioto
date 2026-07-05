//! Core editor functionality for Kioto.
//!
//! Hexagonal shape per `issues/editor-business-requirements.md`: `api`
//! (public surface), `imp` (composition root), `app` (use-case
//! orchestration and document models), `core` (pure editor domain logic),
//! `port` (external capabilities), and `adapters` (side-effecting port and
//! GPUI implementations).
//!
//! The pre-rebuild spike modules were deleted after milestone 5 harvested
//! the last of them (`history.rs` → `core/history.rs`); they live on in
//! git history.

mod adapters;
pub mod api;
mod app;
mod core;
mod imp;
mod port;

pub use api::*;

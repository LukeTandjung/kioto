//! Shared cross-component utilities.
//!
//! Component-specific helpers stay inside each component folder. Direction is exported here because
//! it is ambient behavior context shared by multiple components, such as Radio Group horizontal
//! keyboard navigation, and future direction-aware components should consume the same primitive.

pub mod direction;

pub use direction::{current_direction, DirectionProvider, HorizontalDirection, TextDirection};

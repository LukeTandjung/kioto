//! GPUI-native port of Base UI Progress.
//!
//! Purely presentational compound component: `ProgressRoot` derives clamped
//! value, percentage, `ProgressStatus`, and formatted text once per render;
//! the Track/Indicator/Value/Label parts are thin views of the same derived
//! state via `style_with_state`.
//!
//! Dropped or deferred Base UI props: `role="progressbar"` / ARIA value
//! attributes / `getAriaValueText` (deferred to the AccessKit follow-up),
//! `aria-labelledby` id plumbing, `Intl.NumberFormat` `format`/`locale`
//! options (replaced by a plain `Fn(f64) -> String` callback), and CSS
//! class/style/data-attribute APIs (replaced by `style_with_state` and
//! `ProgressStyleState`). No built-in indeterminate animation — Base UI
//! ships none; indeterminate is status exposure only.

pub mod child;
pub mod context;
mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use child::{ProgressChild, ProgressTrackChild};
pub use context::ProgressContext;
pub use layers::{
    ProgressIndicator, ProgressLabel, ProgressRoot, ProgressTrack, ProgressValue,
    ProgressValueDisplayHandler,
};
pub use props::{ProgressFormatHandler, ProgressProps};
pub use runtime::{ProgressRuntime, ProgressStatus};
pub use style_state::ProgressStyleState;

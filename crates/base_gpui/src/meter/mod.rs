//! GPUI-native port of Base UI Meter.
//!
//! Purely presentational compound component: `MeterRoot` derives clamped
//! value, percentage, and formatted text once per render; the
//! Track/Indicator/Value/Label parts are thin views of the same derived
//! state via `style_with_state`. Unlike Progress, Meter is always
//! determinate — there is no nullable value and no status.
//!
//! Dropped or deferred Base UI props: `role="meter"` / ARIA value attributes
//! / `getAriaValueText` and the visually-hidden NVDA span (deferred to the
//! AccessKit follow-up), `aria-labelledby` id plumbing, `Intl.NumberFormat`
//! `format`/`locale` options (replaced by a plain `Fn(f64) -> String`
//! callback), and CSS class/style/data-attribute APIs (replaced by
//! `style_with_state` and `MeterStyleState`).

pub mod child;
pub mod context;
mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use child::{MeterChild, MeterTrackChild};
pub use context::MeterContext;
pub use layers::{
    MeterIndicator, MeterLabel, MeterRoot, MeterTrack, MeterValue, MeterValueDisplayHandler,
};
pub use props::{MeterFormatHandler, MeterProps};
pub use runtime::MeterRuntime;
pub use style_state::MeterStyleState;

pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
pub mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{init, SwitchToggle, SWITCH_ROOT_KEY_CONTEXT};
pub use child::SwitchChild;
pub use context::SwitchContext;
pub use layers::{SwitchRoot, SwitchThumb};
pub use props::{SwitchCheckedChangeHandler, SwitchProps};
pub use runtime::{
    SwitchCheckedChangeDetails, SwitchCheckedChangeReason, SwitchCheckedChangeSource,
    SwitchRuntime, SwitchToggleOutcome,
};
pub use style_state::{SwitchRootStyleState, SwitchThumbStyleState};

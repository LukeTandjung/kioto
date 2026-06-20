pub mod child;
mod child_wiring;
pub mod context;
pub mod layers;
pub mod props;
pub mod render_state;
pub mod runtime;

#[cfg(test)]
mod tests;

pub use child::CheckboxGroupChild;
pub use context::{current_checkbox_group_context, CheckboxGroupContext};
pub use layers::CheckboxGroup;
pub use props::{CheckboxGroupProps, CheckboxGroupValueChangeHandler};
pub use render_state::CheckboxGroupRenderState;
pub use runtime::{
    CheckboxGroupChildMetadata, CheckboxGroupParentStatus, CheckboxGroupRuntime,
    CheckboxGroupValueChangeDetails, CheckboxGroupValueChangeReason,
    CheckboxGroupValueChangeRequest,
};

pub fn init(_cx: &mut gpui::App) {}

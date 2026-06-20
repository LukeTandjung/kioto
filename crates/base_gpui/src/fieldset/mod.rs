pub mod child;
mod child_wiring;
pub mod context;
pub mod layers;
pub mod props;
pub mod render_state;
pub mod runtime;

#[cfg(test)]
mod tests;

pub use child::FieldsetChild;
pub use context::{current_fieldset_context, current_fieldset_disabled, FieldsetContext};
pub use layers::{FieldsetLegend, FieldsetRoot};
pub use props::FieldsetProps;
pub use render_state::{FieldsetLegendRenderState, FieldsetRootRenderState};
pub use runtime::FieldsetRuntime;

pub fn init(_cx: &mut gpui::App) {}

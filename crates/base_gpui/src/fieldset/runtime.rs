use crate::fieldset::{FieldsetLegendRenderState, FieldsetProps, FieldsetRootRenderState};

#[derive(Clone, Debug, Default)]
pub struct FieldsetRuntime;

impl FieldsetRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Returns the render state for the fieldset group root.
    pub fn root_state(&self, props: &FieldsetProps) -> FieldsetRootRenderState {
        FieldsetRootRenderState::new(props.disabled())
    }

    /// Returns the render state for the fieldset legend.
    pub fn legend_state(&self, props: &FieldsetProps) -> FieldsetLegendRenderState {
        FieldsetLegendRenderState::new(self.root_state(props))
    }
}

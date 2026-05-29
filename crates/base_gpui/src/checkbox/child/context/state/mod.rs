mod checkbox_state;

pub use checkbox_state::CheckboxState;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CheckboxRootRenderState {
    pub checked: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub indeterminate: bool,
}

impl CheckboxRootRenderState {
    pub fn new(
        checked: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        indeterminate: bool,
    ) -> Self {
        Self {
            checked,
            disabled,
            read_only,
            required,
            indeterminate,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckboxIndicatorRenderState {
    pub root: CheckboxRootRenderState,
    pub present: bool,
}

impl CheckboxIndicatorRenderState {
    pub fn new(root: CheckboxRootRenderState, keep_mounted: bool) -> Self {
        Self {
            present: keep_mounted || root.checked || root.indeterminate,
            root,
        }
    }
}

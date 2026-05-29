mod checkbox_state;

pub use checkbox_state::CheckboxState;

use gpui::SharedString;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CheckboxRootRenderState {
    pub checked: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub indeterminate: bool,
    pub name: Option<SharedString>,
    pub value: Option<SharedString>,
    pub form: Option<SharedString>,
    pub parent: bool,
    pub unchecked_value: Option<SharedString>,
}

impl CheckboxRootRenderState {
    pub fn new(
        checked: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        indeterminate: bool,
        name: Option<SharedString>,
        value: Option<SharedString>,
        form: Option<SharedString>,
        parent: bool,
        unchecked_value: Option<SharedString>,
    ) -> Self {
        Self {
            checked,
            disabled,
            read_only,
            required,
            indeterminate,
            name,
            value,
            form,
            parent,
            unchecked_value,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

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

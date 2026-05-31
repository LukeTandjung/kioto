#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckboxRootRenderState {
    pub checked: bool,
    pub unchecked: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub indeterminate: bool,
    pub focused: bool,
}

impl CheckboxRootRenderState {
    pub fn new(
        checked: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        indeterminate: bool,
        focused: bool,
    ) -> Self {
        Self {
            checked,
            unchecked: !checked && !indeterminate,
            disabled,
            read_only,
            required,
            indeterminate,
            focused,
        }
    }
}

impl Default for CheckboxRootRenderState {
    fn default() -> Self {
        Self::new(false, false, false, false, false, false)
    }
}

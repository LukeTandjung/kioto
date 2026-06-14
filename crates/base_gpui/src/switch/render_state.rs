#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SwitchRootRenderState {
    pub checked: bool,
    pub unchecked: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub focused: bool,
}

impl SwitchRootRenderState {
    pub fn new(
        checked: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        focused: bool,
    ) -> Self {
        Self {
            checked,
            unchecked: !checked,
            disabled,
            read_only,
            required,
            focused,
        }
    }
}

impl Default for SwitchRootRenderState {
    fn default() -> Self {
        Self::new(false, false, false, false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SwitchThumbRenderState {
    pub root: SwitchRootRenderState,
}

impl SwitchThumbRenderState {
    pub fn new(root: SwitchRootRenderState) -> Self {
        Self { root }
    }
}

impl Default for SwitchThumbRenderState {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

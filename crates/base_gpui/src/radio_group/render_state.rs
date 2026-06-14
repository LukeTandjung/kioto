#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadioGroupRootRenderState {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub focused: bool,
    pub filled: bool,
}

impl RadioGroupRootRenderState {
    pub fn new(
        disabled: bool,
        read_only: bool,
        required: bool,
        focused: bool,
        filled: bool,
    ) -> Self {
        Self {
            disabled,
            read_only,
            required,
            focused,
            filled,
        }
    }
}

impl Default for RadioGroupRootRenderState {
    fn default() -> Self {
        Self::new(false, false, false, false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadioGroupRadioRenderState {
    pub checked: bool,
    pub unchecked: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub focused: bool,
    pub highlighted: bool,
    pub tab_stop: bool,
}

impl RadioGroupRadioRenderState {
    pub fn new(
        checked: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        focused: bool,
        highlighted: bool,
        tab_stop: bool,
    ) -> Self {
        Self {
            checked,
            unchecked: !checked,
            disabled,
            read_only,
            required,
            focused,
            highlighted,
            tab_stop,
        }
    }
}

impl Default for RadioGroupRadioRenderState {
    fn default() -> Self {
        Self::new(false, false, false, false, false, false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadioGroupIndicatorRenderState {
    pub radio: RadioGroupRadioRenderState,
    pub present: bool,
}

impl RadioGroupIndicatorRenderState {
    pub fn new(radio: RadioGroupRadioRenderState, present: bool) -> Self {
        Self { radio, present }
    }
}

impl Default for RadioGroupIndicatorRenderState {
    fn default() -> Self {
        Self::new(Default::default(), false)
    }
}

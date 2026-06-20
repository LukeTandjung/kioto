use gpui::SharedString;

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldRootRenderState {
    pub value: Option<f64>,
    pub input_value: SharedString,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub scrubbing: bool,
    pub touched: bool,
    pub dirty: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
    pub filled: bool,
    pub focused: bool,
    pub controlled: bool,
}

impl NumberFieldRootRenderState {
    pub fn new(
        value: Option<f64>,
        input_value: SharedString,
        disabled: bool,
        read_only: bool,
        required: bool,
        scrubbing: bool,
        touched: bool,
        dirty: bool,
        valid: Option<bool>,
        focused: bool,
        controlled: bool,
    ) -> Self {
        Self {
            value,
            input_value,
            disabled,
            read_only,
            required,
            scrubbing,
            touched,
            dirty,
            valid,
            invalid: valid == Some(false),
            filled: value.is_some(),
            focused,
            controlled,
        }
    }
}

impl Default for NumberFieldRootRenderState {
    fn default() -> Self {
        Self::new(
            None,
            SharedString::default(),
            false,
            false,
            false,
            false,
            false,
            false,
            None,
            false,
            false,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldInputRenderState {
    pub root: NumberFieldRootRenderState,
}

impl NumberFieldInputRenderState {
    pub fn new(root: NumberFieldRootRenderState) -> Self {
        Self { root }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldGroupRenderState {
    pub root: NumberFieldRootRenderState,
}

impl NumberFieldGroupRenderState {
    pub fn new(root: NumberFieldRootRenderState) -> Self {
        Self { root }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldIncrementRenderState {
    pub root: NumberFieldRootRenderState,
    pub can_increment: bool,
}

impl NumberFieldIncrementRenderState {
    pub fn new(root: NumberFieldRootRenderState, can_increment: bool) -> Self {
        Self {
            root,
            can_increment,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldDecrementRenderState {
    pub root: NumberFieldRootRenderState,
    pub can_decrement: bool,
}

impl NumberFieldDecrementRenderState {
    pub fn new(root: NumberFieldRootRenderState, can_decrement: bool) -> Self {
        Self {
            root,
            can_decrement,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldScrubAreaRenderState {
    pub root: NumberFieldRootRenderState,
    pub horizontal: bool,
    pub vertical: bool,
}

impl NumberFieldScrubAreaRenderState {
    pub fn new(root: NumberFieldRootRenderState, horizontal: bool, vertical: bool) -> Self {
        Self {
            root,
            horizontal,
            vertical,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldScrubAreaCursorRenderState {
    pub root: NumberFieldRootRenderState,
    pub present: bool,
}

impl NumberFieldScrubAreaCursorRenderState {
    pub fn new(root: NumberFieldRootRenderState, present: bool) -> Self {
        Self { root, present }
    }
}

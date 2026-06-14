use crate::field::validation::FieldValidityData;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldRootRenderState {
    pub disabled: bool,
    pub touched: bool,
    pub dirty: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
    pub filled: bool,
    pub focused: bool,
}

impl FieldRootRenderState {
    pub fn new(
        disabled: bool,
        touched: bool,
        dirty: bool,
        valid: Option<bool>,
        filled: bool,
        focused: bool,
    ) -> Self {
        Self {
            disabled,
            touched,
            dirty,
            valid,
            invalid: valid == Some(false),
            filled,
            focused,
        }
    }
}

impl Default for FieldRootRenderState {
    fn default() -> Self {
        Self::new(false, false, false, None, false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldItemRenderState {
    pub root: FieldRootRenderState,
    pub disabled: bool,
}

impl FieldItemRenderState {
    pub fn new(root: FieldRootRenderState, disabled: bool) -> Self {
        Self { root, disabled }
    }
}

impl Default for FieldItemRenderState {
    fn default() -> Self {
        Self::new(Default::default(), false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldLabelRenderState {
    pub root: FieldRootRenderState,
    pub disabled: bool,
}

impl FieldLabelRenderState {
    pub fn new(root: FieldRootRenderState, disabled: bool) -> Self {
        Self { root, disabled }
    }
}

impl Default for FieldLabelRenderState {
    fn default() -> Self {
        Self::new(Default::default(), false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldDescriptionRenderState {
    pub root: FieldRootRenderState,
    pub disabled: bool,
}

impl FieldDescriptionRenderState {
    pub fn new(root: FieldRootRenderState, disabled: bool) -> Self {
        Self { root, disabled }
    }
}

impl Default for FieldDescriptionRenderState {
    fn default() -> Self {
        Self::new(Default::default(), false)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldErrorRenderState {
    pub root: FieldRootRenderState,
    pub present: bool,
    pub errors: Vec<gpui::SharedString>,
    pub error: gpui::SharedString,
}

impl FieldErrorRenderState {
    pub fn new(
        root: FieldRootRenderState,
        present: bool,
        errors: Vec<gpui::SharedString>,
        error: gpui::SharedString,
    ) -> Self {
        Self {
            root,
            present,
            errors,
            error,
        }
    }
}

impl Default for FieldErrorRenderState {
    fn default() -> Self {
        Self::new(Default::default(), false, Vec::new(), Default::default())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldValidityRenderState {
    pub root: FieldRootRenderState,
    pub validity: FieldValidityData,
}

impl FieldValidityRenderState {
    pub fn new(root: FieldRootRenderState, validity: FieldValidityData) -> Self {
        Self { root, validity }
    }
}

impl Default for FieldValidityRenderState {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

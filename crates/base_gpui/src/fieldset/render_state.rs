#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldsetRootRenderState {
    pub disabled: bool,
}

impl FieldsetRootRenderState {
    pub fn new(disabled: bool) -> Self {
        Self { disabled }
    }
}

impl Default for FieldsetRootRenderState {
    fn default() -> Self {
        Self::new(false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldsetLegendRenderState {
    pub root: FieldsetRootRenderState,
    pub disabled: bool,
}

impl FieldsetLegendRenderState {
    pub fn new(root: FieldsetRootRenderState) -> Self {
        Self {
            disabled: root.disabled,
            root,
        }
    }
}

impl Default for FieldsetLegendRenderState {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

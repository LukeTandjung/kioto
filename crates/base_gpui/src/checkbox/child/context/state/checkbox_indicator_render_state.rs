use super::CheckboxRootRenderState;

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

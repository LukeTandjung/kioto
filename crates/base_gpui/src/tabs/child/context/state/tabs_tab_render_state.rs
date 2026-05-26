use crate::tabs::TabsOrientation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsTabRenderState {
    pub active: bool,
    pub disabled: bool,
    pub highlighted: bool,
    pub orientation: TabsOrientation,
}

impl TabsTabRenderState {
    pub fn new(
        active: bool,
        disabled: bool,
        highlighted: bool,
        orientation: TabsOrientation,
    ) -> Self {
        Self {
            active,
            disabled,
            highlighted,
            orientation,
        }
    }
}

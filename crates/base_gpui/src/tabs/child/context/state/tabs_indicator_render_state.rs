use crate::tabs::{TabsActivationDirection, TabsOrientation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsIndicatorRenderState {
    pub selected: bool,
    pub orientation: TabsOrientation,
    pub activation_direction: TabsActivationDirection,
}

impl TabsIndicatorRenderState {
    pub fn new(
        selected: bool,
        orientation: TabsOrientation,
        activation_direction: TabsActivationDirection,
    ) -> Self {
        Self {
            selected,
            orientation,
            activation_direction,
        }
    }
}

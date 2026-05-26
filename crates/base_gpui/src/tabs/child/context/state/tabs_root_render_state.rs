use crate::tabs::{TabsActivationDirection, TabsOrientation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsRootRenderState {
    pub orientation: TabsOrientation,
    pub activation_direction: TabsActivationDirection,
}

impl TabsRootRenderState {
    pub fn new(
        orientation: TabsOrientation,
        activation_direction: TabsActivationDirection,
    ) -> Self {
        Self {
            orientation,
            activation_direction,
        }
    }
}

use crate::tabs::{TabsActivationDirection, TabsOrientation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsListRenderState {
    pub orientation: TabsOrientation,
    pub activation_direction: TabsActivationDirection,
}

impl TabsListRenderState {
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

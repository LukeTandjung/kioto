use crate::tabs::{TabsActivationDirection, TabsOrientation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsPanelRenderState {
    pub hidden: bool,
    pub orientation: TabsOrientation,
    pub activation_direction: TabsActivationDirection,
}

impl TabsPanelRenderState {
    pub fn new(
        hidden: bool,
        orientation: TabsOrientation,
        activation_direction: TabsActivationDirection,
    ) -> Self {
        Self {
            hidden,
            orientation,
            activation_direction,
        }
    }
}

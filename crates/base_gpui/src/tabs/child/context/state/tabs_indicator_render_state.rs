use crate::tabs::{TabsActivationDirection, TabsOrientation, TabsTabPosition, TabsTabSize};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabsIndicatorRenderState {
    pub selected: bool,
    pub active_tab_position: Option<TabsTabPosition>,
    pub active_tab_size: Option<TabsTabSize>,
    pub orientation: TabsOrientation,
    pub activation_direction: TabsActivationDirection,
}

impl TabsIndicatorRenderState {
    pub fn new(
        selected: bool,
        active_tab_position: Option<TabsTabPosition>,
        active_tab_size: Option<TabsTabSize>,
        orientation: TabsOrientation,
        activation_direction: TabsActivationDirection,
    ) -> Self {
        Self {
            selected,
            active_tab_position,
            active_tab_size,
            orientation,
            activation_direction,
        }
    }
}

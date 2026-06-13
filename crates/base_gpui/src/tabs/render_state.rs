use crate::tabs::{TabsActivationDirection, TabsOrientation, TabsTabPosition, TabsTabSize};

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

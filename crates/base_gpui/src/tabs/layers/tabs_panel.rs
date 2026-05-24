use gpui::{
    AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
    div,
};

use crate::{tabs::TabsState, api::GenericChild, utils::ControlledState};

#[derive(IntoElement)]
pub struct TabsPanel<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    state: Option<ControlledState<TabsState<T>>>,
    value: Option<T>,
    keep_mounted: bool,
}

impl<T: Clone + Eq + 'static> Default for TabsPanel<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            state: None,
            value: None,
            keep_mounted: false,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for TabsPanel<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsPanel<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsPanel<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

impl<T: Clone + Eq + 'static> GenericChild<ControlledState<TabsState<T>>> for TabsPanel<T> {
    fn add_state_context(mut self, state: ControlledState<TabsState<T>>) -> Self {
        self.state = Some(state);
        self
    }
}

impl<T: Clone + Eq + 'static> TabsPanel<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }
}

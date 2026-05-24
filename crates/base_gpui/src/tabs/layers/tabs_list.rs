use gpui::{
    App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window, div,
};

use crate::{tabs::TabsState, api::GenericChild, utils::ControlledState};

use super::TabsTab;

#[derive(IntoElement)]
pub struct TabsList<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<TabsTab<T>>,
    state: Option<ControlledState<TabsState<T>>>,
    activate_on_focus: bool,
    loop_focus: bool,
}

impl<T: Clone + Eq + 'static> Default for TabsList<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            state: None,
            activate_on_focus: false,
            loop_focus: true,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsList<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let state = self.state;
        self.base.children(self.children.into_iter().map(|tab| {
            match state.clone() {
                Some(state) => tab.add_state_context(state).into_any_element(),
                None => tab.into_any_element(),
            }
        }))
    }
}

impl<T: Clone + Eq + 'static> GenericChild<ControlledState<TabsState<T>>> for TabsList<T> {
    fn add_state_context(mut self, state: ControlledState<TabsState<T>>) -> Self {
        self.state = Some(state);
        self
    }
}

impl<T: Clone + Eq + 'static> TabsList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, tab: TabsTab<T>) -> Self {
        self.children.push(tab);
        self
    }

    pub fn children(mut self, tabs: impl IntoIterator<Item = TabsTab<T>>) -> Self {
        self.children.extend(tabs);
        self
    }

    pub fn activate_on_focus(mut self, activate_on_focus: bool) -> Self {
        self.activate_on_focus = activate_on_focus;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }
}

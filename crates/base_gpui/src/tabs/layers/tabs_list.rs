use gpui::{
    App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window, div,
};

use crate::{
    api::GenericChild,
    tabs::{TabsProps, TabsRuntime, TabsState},
    utils::ControlledContext,
};

use super::TabsTab;

#[derive(IntoElement)]
pub struct TabsList<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<TabsTab<T>>,
    context: Option<ControlledContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>>,
    activate_on_focus: bool,
    loop_focus: bool,
}

impl<T: Clone + Eq + 'static> Default for TabsList<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            context: None,
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
        let context = self.context;
        self.base.children(self.children.into_iter().map(|tab| {
            match context.clone() {
                Some(context) => tab.add_state_context(context).into_any_element(),
                None => tab.into_any_element(),
            }
        }))
    }
}

impl<T: Clone + Eq + 'static> GenericChild<ControlledContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>>
    for TabsList<T>
{
    fn add_state_context(mut self, context: ControlledContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>) -> Self {
        self.context = Some(context);
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

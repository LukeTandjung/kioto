use gpui::{
    prelude::FluentBuilder as _, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window, div,
};

use crate::{
    api::GenericChild,
    tabs::{TabsProps, TabsRuntime, TabsState},
    utils::ControlledContext,
};

#[derive(IntoElement)]
pub struct TabsPanel<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ControlledContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>>,
    value: Option<T>,
    keep_mounted: bool,
}

impl<T: Clone + Eq + 'static> Default for TabsPanel<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            context: None,
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let selected = self
            .context
            .as_ref()
            .and_then(|context| context.selected_value(cx));
        let active = match (self.value.as_ref(), selected.as_ref()) {
            (Some(value), Some(selected)) => value == selected,
            _ => false,
        };
        let hidden = !active;
        let _orientation = self
            .context
            .as_ref()
            .map(|context| context.props().orientation());

        if active || self.keep_mounted {
            self.base
                .children(self.children)
                .when(hidden, |this| this.invisible())
                .into_any_element()
        } else {
            div().into_any_element()
        }
    }
}

impl<T: Clone + Eq + 'static> GenericChild<ControlledContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>>
    for TabsPanel<T>
{
    fn add_state_context(mut self, context: ControlledContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>) -> Self {
        self.context = Some(context);
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

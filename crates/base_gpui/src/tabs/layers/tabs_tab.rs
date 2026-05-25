use gpui::{
    prelude::FluentBuilder as _, AnyElement, App, Div, ElementId, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window, div,
};

use crate::{
    api::GenericChild,
    tabs::{TabsProps, TabsState},
    utils::ControlledContext,
};

#[derive(IntoElement)]
pub struct TabsTab<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ControlledContext<TabsState<T>, TabsProps<T>>>,
    value: Option<T>,
    disabled: bool,
}

impl<T: Clone + Eq + 'static> Default for TabsTab<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("tabs-tab"),
            base: div(),
            children: Vec::from([]),
            context: None,
            value: None,
            disabled: false,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for TabsTab<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsTab<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsTab<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self.context;
        let value = self.value;
        let disabled = self.disabled;

        self.base
            .id(self.id)
            .children(self.children)
            .when(!disabled, |this| {
                this.when_some(context.zip(value), |this, (context, value)| {
                    this.on_click(move |event, window, cx| {
                        context.select_value(Some(value.clone()), event, window, cx);
                    })
                })
            })
    }
}

impl<T: Clone + Eq + 'static> GenericChild<ControlledContext<TabsState<T>, TabsProps<T>>>
    for TabsTab<T>
{
    fn add_state_context(mut self, context: ControlledContext<TabsState<T>, TabsProps<T>>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> TabsTab<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

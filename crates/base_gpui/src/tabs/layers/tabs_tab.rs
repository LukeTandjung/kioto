use gpui::{
    prelude::FluentBuilder as _, AnyElement, App, Div, ElementId, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window, div,
};

use crate::{
    api::GenericChild,
    tabs::{TabsContext, TabsRuntime},
};

#[derive(IntoElement)]
pub struct TabsTab<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<TabsContext<T>>,
    value: Option<T>,
    disabled: bool,
    index: Option<usize>,
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
            index: None,
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            base,
            children,
            context,
            value,
            disabled,
            index,
        } = self;

        let selected_value = context.as_ref().and_then(|context| context.selected_value(cx));
        let active = match (value.as_ref(), selected_value.as_ref()) {
            (Some(value), Some(selected_value)) => value == selected_value,
            _ => false,
        };
        let _orientation = context.as_ref().map(|context| context.props().orientation());
        let _highlighted = context.as_ref().is_some_and(|context| {
            context.get_runtime(cx, |runtime| runtime.highlighted_tab_index() == index)
        });

        let selectable = match !disabled && !active {
            true => context.zip(value).zip(index),
            false => None,
        };

        base.id(id)
            .children(children)
            .when_some(selectable, |this, ((context, value), index)| {
                this.on_click(move |event, window, cx| {
                    context.highlight_tab(Some(index), cx);
                    context.select_value(Some(value.clone()), event, window, cx);
                })
            })
    }
}

impl<T: Clone + Eq + 'static>
    GenericChild<TabsContext<T>> for TabsTab<T>
{
    fn add_state_context(
        mut self,
        context: TabsContext<T>,
    ) -> Self {
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

    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn register_runtime(&self, index: usize, runtime: &mut TabsRuntime<T>) {
        if let Some(value) = self.value.as_ref() {
            runtime.register_tab(value.clone(), self.disabled, index);
        }
    }
}

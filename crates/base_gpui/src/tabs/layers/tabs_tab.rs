use std::{rc::Rc, sync::Arc};

use gpui::{
    prelude::FluentBuilder as _, AnyElement, App, ClickEvent, Div, ElementId, Entity,
    FocusHandle, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
};

use crate::{
    api::GenericChild,
    tabs::{TabsContext, TabsOrientation, TabsTabRenderState},
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
    style_with_state: Option<Rc<dyn Fn(TabsTabRenderState, Div) -> Div + 'static>>,
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
            style_with_state: None,
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
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            base,
            children,
            context,
            value,
            disabled,
            index,
            style_with_state,
        } = self;

        let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
            cx,
            |_, cx| cx.focus_handle(),
        );
        let focus_handle = focus_handle_entity.read(cx).clone();

        let focused = focus_handle.is_focused(window);
        let state = context
            .as_ref()
            .map(|context| {
                context.tab_render_state(value.as_ref(), disabled, index, focused, cx)
            })
            .unwrap_or_else(|| {
                TabsTabRenderState::new(false, disabled, false, focused, TabsOrientation::Horizontal)
            });
        let active = state.active;
        let highlighted = state.highlighted;

        let selectable = match !disabled && !active {
            true => context.zip(value).zip(index),
            false => None,
        };

        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        base.id(id)
            .track_focus(
                &focus_handle
                    .tab_stop(highlighted && !disabled)
                    .tab_index(if highlighted { 0 } else { -1 }),
            )
            .children(children)
            .when_some(selectable, |this, ((context, value), index)| {
                this.on_click(move |event, window, cx| {
                    if !matches!(event, ClickEvent::Mouse(_)) {
                        return;
                    }

                    context.highlight_tab(Some(index), cx);
                    context.select_value(Some(value.clone()), window, cx);
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

    pub fn style_with_state(
        mut self,
        style: impl Fn(TabsTabRenderState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    pub fn register_runtime(
        &self,
        index: usize,
        context: &TabsContext<T>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(value) = self.value.as_ref() {
            context.register_tab(value.clone(), self.disabled, index, cx);
        }

        let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(self.id.clone()), SharedString::from("focus")),
            cx,
            |_, cx| cx.focus_handle(),
        );
        let focus_handle = focus_handle_entity.read(cx).clone();

        context.register_tab_focus_handle(index, focus_handle, cx);
    }
}

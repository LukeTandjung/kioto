use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, Div, Empty, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::{
    api::GenericChild,
    tabs::{TabsContext, TabsOrientation, TabsPanelRenderState},
};

#[derive(IntoElement)]
pub struct TabsPanel<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<TabsContext<T>>,
    value: Option<T>,
    keep_mounted: bool,
    index: Option<usize>,
    style_with_state: Option<Rc<dyn Fn(TabsPanelRenderState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for TabsPanel<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            context: None,
            value: None,
            keep_mounted: false,
            index: None,
            style_with_state: None,
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
        let Self {
            base,
            children,
            context,
            value,
            keep_mounted,
            index: _index,
            style_with_state,
        } = self;

        let state = context
            .as_ref()
            .map(|context| context.panel_render_state(value.as_ref(), cx))
            .unwrap_or_else(|| {
                TabsPanelRenderState::new(true, TabsOrientation::Horizontal, Default::default())
            });
        let active = !state.hidden;
        let hidden = state.hidden;
        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        if active || keep_mounted {
            base.children(children)
                .when(hidden, |this| this.invisible())
                .into_any_element()
        } else {
            Empty.into_any_element()
        }
    }
}

impl<T: Clone + Eq + 'static> GenericChild<TabsContext<T>> for TabsPanel<T> {
    fn add_state_context(mut self, context: TabsContext<T>) -> Self {
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

    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TabsPanelRenderState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    pub fn register_runtime(&self, index: usize, context: &TabsContext<T>, cx: &mut App) {
        if let Some(value) = self.value.as_ref() {
            context.register_panel(value.clone(), index, cx);
        }
    }
}

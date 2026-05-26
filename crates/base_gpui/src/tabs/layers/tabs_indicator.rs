use std::{marker::PhantomData, rc::Rc};

use gpui::{
    AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
    div,
};

use crate::{
    api::GenericChild,
    tabs::{TabsContext, TabsIndicatorRenderState, TabsOrientation},
};

#[derive(IntoElement)]
pub struct TabsIndicator<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<TabsContext<T>>,
    style_with_state: Option<Rc<dyn Fn(TabsIndicatorRenderState, Div) -> Div + 'static>>,
    tab_value: PhantomData<T>,
}

impl<T: Clone + Eq + 'static> Default for TabsIndicator<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            context: None,
            style_with_state: None,
            tab_value: PhantomData,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for TabsIndicator<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsIndicator<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsIndicator<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            base,
            children,
            context,
            style_with_state,
            tab_value: _,
        } = self;

        let state = context
            .as_ref()
            .map(|context| context.indicator_render_state(cx))
            .unwrap_or_else(|| {
                TabsIndicatorRenderState::new(
                    false,
                    None,
                    None,
                    TabsOrientation::Horizontal,
                    Default::default(),
                )
            });

        if !state.selected {
            return div().into_any_element();
        }

        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        base.children(children).into_any_element()
    }
}

impl<T: Clone + Eq + 'static> GenericChild<TabsContext<T>> for TabsIndicator<T> {
    fn add_state_context(mut self, context: TabsContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> TabsIndicator<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TabsIndicatorRenderState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

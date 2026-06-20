use std::rc::Rc;

use gpui::{
    div, AnyElement, App, ClickEvent, Div, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::number_field::{
    NumberFieldChangeReason, NumberFieldCommitReason, NumberFieldContext,
    NumberFieldDecrementRenderState, NumberFieldStepAmount, NumberFieldStepDirection,
};

#[derive(IntoElement)]
pub struct NumberFieldDecrement {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<NumberFieldContext>,
    style_with_state: Option<Rc<dyn Fn(NumberFieldDecrementRenderState, Div) -> Div + 'static>>,
}

impl Default for NumberFieldDecrement {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl Styled for NumberFieldDecrement {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for NumberFieldDecrement {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("NumberFieldDecrement must be rendered inside NumberFieldRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("decrement"));
        let render_state = context.read(cx, |runtime, props| runtime.decrement_state(props));
        let click_context = context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(render_state, self.base),
            None => self.base,
        };

        base.id(id)
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                click_context.step(
                    NumberFieldStepDirection::Down,
                    NumberFieldStepAmount::Normal,
                    NumberFieldChangeReason::DecrementPress,
                    NumberFieldCommitReason::DecrementPress,
                    window,
                    cx,
                );
            })
            .children(self.children)
    }
}

impl NumberFieldDecrement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_number_field_context(mut self, context: NumberFieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NumberFieldDecrementRenderState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

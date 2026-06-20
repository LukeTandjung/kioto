use std::rc::Rc;

use gpui::{
    App, Div, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::{
    field::{current_field_context, FieldContext, FieldControl},
    input::InputStyleState,
};

#[derive(IntoElement)]
pub struct Input {
    control: FieldControl,
    context: Option<FieldContext>,
    style_with_state: Option<Rc<dyn Fn(InputStyleState, Div) -> Div + 'static>>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            control: FieldControl::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl Styled for Input {
    fn style(&mut self) -> &mut StyleRefinement {
        self.control.style()
    }
}

impl RenderOnce for Input {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self.context.clone().or_else(current_field_context);
        let field_state = context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)));
        let style_with_state = self.style_with_state;

        self.control
            .style_with_state(move |primitive_state, input| {
                let state = InputStyleState::new(primitive_state, field_state);

                match style_with_state.as_ref() {
                    Some(style_with_state) => style_with_state(state, input),
                    None => input,
                }
            })
    }
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.control = self.control.with_field_context(context.clone());
        self.context = Some(context);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.control = self.control.id(id);
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.control = self.control.name(name);
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.control = self.control.value(value);
        self
    }

    pub fn default_value(mut self, default_value: impl Into<SharedString>) -> Self {
        self.control = self.control.default_value(default_value);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.control = self.control.placeholder(placeholder);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.control = self.control.disabled(disabled);
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.control = self.control.read_only(read_only);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.control = self.control.required(required);
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.control = self.control.auto_focus(auto_focus);
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.control = self.control.tab_index(tab_index);
        self
    }

    pub fn on_value_change(mut self, on_value_change: impl Fn(SharedString) + 'static) -> Self {
        self.control = self.control.on_value_change(on_value_change);
        self
    }

    pub fn on_enter(mut self, on_enter: impl Fn(SharedString) + 'static) -> Self {
        self.control = self.control.on_enter(on_enter);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(InputStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

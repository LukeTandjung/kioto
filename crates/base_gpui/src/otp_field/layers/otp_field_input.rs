use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, MouseButton, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::otp_field::{layers::OtpSlotElement, OTPFieldContext, OTPFieldInputStyleState};

#[derive(IntoElement)]
pub struct OTPFieldInput {
    id: Option<ElementId>,
    base: Div,
    context: Option<OTPFieldContext>,
    slot_index: usize,
    style_with_state: Option<Rc<dyn Fn(OTPFieldInputStyleState, Div) -> Div + 'static>>,
}

impl Default for OTPFieldInput {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            context: None,
            slot_index: 0,
            style_with_state: None,
        }
    }
}

impl Styled for OTPFieldInput {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for OTPFieldInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("OTPFieldInput must be rendered inside OTPFieldRoot");
        let index = self.slot_index;
        let id = self
            .id
            .unwrap_or_else(|| context.child_id(format!("slot-{index}")));
        let valid = context.field_valid();
        let state = context.read(cx, |runtime, props| {
            runtime.input_state(index, props, valid)
        });

        // Slots beyond `length` are not editable.
        let editable = index < state.root.length;
        let disabled = state.root.disabled;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state.clone(), self.base),
            None => self.base,
        };
        let mouse_context = context.clone();

        base.id(id)
            .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                if !disabled && editable {
                    mouse_context.activate_slot(index, window, cx);
                }
            })
            .child(OtpSlotElement::new(
                context,
                index,
                state.value.clone(),
                state.masked,
                state.active && editable,
                disabled,
            ))
    }
}

impl OTPFieldInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_otp_field_context(mut self, context: OTPFieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_slot_index(mut self, index: usize) -> Self {
        self.slot_index = index;
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(OTPFieldInputStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

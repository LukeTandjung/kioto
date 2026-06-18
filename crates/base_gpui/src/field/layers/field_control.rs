use std::{rc::Rc, sync::Arc};

use gpui::{
    App, Div, ElementId, Entity, FocusHandle, IntoElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

use crate::{
    field::{
        context::current_field_context, current_field_item_disabled, FieldContext,
        FieldControlRegistration, FieldValue,
    },
    primitives::input::{input, Input, InputRenderState},
};

#[derive(IntoElement)]
pub struct FieldControl {
    id: ElementId,
    input: Input,
    context: Option<FieldContext>,
    name: Option<SharedString>,
    disabled: bool,
    required: bool,
    style_with_state: Option<Rc<dyn Fn(InputRenderState, Div) -> Div + 'static>>,
}

impl Default for FieldControl {
    fn default() -> Self {
        let id = ElementId::from("field-control");

        Self {
            input: input().id(id.clone()),
            id,
            context: None,
            name: None,
            disabled: false,
            required: false,
            style_with_state: None,
        }
    }
}

impl Styled for FieldControl {
    fn style(&mut self) -> &mut StyleRefinement {
        self.input.style()
    }
}

impl RenderOnce for FieldControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = self.context.or_else(current_field_context);
        let field_state = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)));
        let field_disabled = field_state.map(|state| state.disabled).unwrap_or(false);
        let field_valid = field_state.and_then(|state| state.valid);
        let item_disabled = current_field_item_disabled();
        let disabled = self.disabled || field_disabled || item_disabled;

        let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(self.id.clone()), SharedString::from("focus")),
            cx,
            |_, cx| cx.focus_handle(),
        );
        let focus_handle = focus_handle_entity.read(cx).clone();

        let registration_context = field_context.clone();
        let registration_id = self.id.to_string();
        let registration_name = self.name.clone();
        let registration_focus_handle = focus_handle.clone();
        let style_with_state = self.style_with_state;

        self.input
            .disabled(disabled)
            .focus_handle(focus_handle)
            .on_render_state(move |state, _window, cx| {
                if let Some(context) = registration_context.as_ref() {
                    let mut registration = FieldControlRegistration::new(registration_id.clone())
                        .value(FieldValue::Text(state.value.clone()))
                        .disabled(state.disabled)
                        .focused(state.focused)
                        .required(state.required)
                        .focus_handle(registration_focus_handle.clone());

                    if let Some(name) = registration_name.clone() {
                        registration = registration.name(name);
                    }

                    context.register_control(registration, cx);
                }
            })
            .style_with_state(move |mut state, input| {
                state.valid = field_valid;
                state.invalid = field_valid == Some(false);

                match style_with_state.as_ref() {
                    Some(style_with_state) => style_with_state(state, input),
                    None => input,
                }
            })
    }
}

impl FieldControl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        let id = id.into();
        self.input = self.input.id(id.clone());
        self.id = id;
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        let name = name.into();
        self.input = self.input.name(name.clone());
        self.name = Some(name);
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.input = self.input.value(value);
        self
    }

    pub fn default_value(mut self, default_value: impl Into<SharedString>) -> Self {
        self.input = self.input.default_value(default_value);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.input = self.input.placeholder(placeholder);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.input = self.input.disabled(disabled);
        self.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.input = self.input.read_only(read_only);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.input = self.input.required(required);
        self.required = required;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.input = self.input.auto_focus(auto_focus);
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.input = self.input.tab_index(tab_index);
        self
    }

    pub fn on_value_change(mut self, on_value_change: impl Fn(SharedString) + 'static) -> Self {
        self.input = self.input.on_value_change(on_value_change);
        self
    }

    pub fn on_enter(mut self, on_enter: impl Fn(SharedString) + 'static) -> Self {
        self.input = self.input.on_enter(on_enter);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(InputRenderState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

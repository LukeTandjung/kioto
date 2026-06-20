use gpui::SharedString;

use crate::{
    field::FieldRootRenderState, primitives::input::InputRenderState as PrimitiveInputRenderState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputRenderState {
    pub value: SharedString,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub focused: bool,
    pub empty: bool,
    pub filled: bool,
    pub dirty: bool,
    pub touched: bool,
    pub controlled: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
}

impl InputRenderState {
    pub fn new(primitive: PrimitiveInputRenderState, field: Option<FieldRootRenderState>) -> Self {
        let valid = field.map_or(primitive.valid, |field| field.valid);
        let filled = field.map_or(primitive.filled, |field| field.filled);
        let dirty = field.map_or(primitive.dirty, |field| field.dirty);
        let focused = field.map_or(primitive.focused, |field| field.focused);
        let touched = field.map_or(false, |field| field.touched);

        Self {
            value: primitive.value,
            disabled: primitive.disabled,
            read_only: primitive.read_only,
            required: primitive.required,
            focused,
            empty: !filled,
            filled,
            dirty,
            touched,
            controlled: primitive.controlled,
            valid,
            invalid: valid == Some(false),
        }
    }
}

impl Default for InputRenderState {
    fn default() -> Self {
        Self::new(PrimitiveInputRenderState::default(), None)
    }
}

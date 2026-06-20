use gpui::{AnyElement, IntoElement};

use crate::{
    field::{FieldControl, FieldDescription, FieldError, FieldItem, FieldLabel, FieldValidity},
    number_field::NumberFieldRoot,
};

pub enum FieldChild {
    Item(FieldItem),
    Label(FieldLabel),
    Control(FieldControl),
    Description(FieldDescription),
    Error(FieldError),
    Validity(FieldValidity),
    NumberField(NumberFieldRoot),
    Any(AnyElement),
}

impl IntoElement for FieldChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Item(item) => item.into_any_element(),
            Self::Label(label) => label.into_any_element(),
            Self::Control(control) => control.into_any_element(),
            Self::Description(description) => description.into_any_element(),
            Self::Error(error) => error.into_any_element(),
            Self::Validity(validity) => validity.into_any_element(),
            Self::NumberField(number_field) => number_field.into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl From<FieldItem> for FieldChild {
    fn from(value: FieldItem) -> Self {
        Self::Item(value)
    }
}

impl From<FieldLabel> for FieldChild {
    fn from(value: FieldLabel) -> Self {
        Self::Label(value)
    }
}

impl From<FieldControl> for FieldChild {
    fn from(value: FieldControl) -> Self {
        Self::Control(value)
    }
}

impl From<FieldDescription> for FieldChild {
    fn from(value: FieldDescription) -> Self {
        Self::Description(value)
    }
}

impl From<FieldError> for FieldChild {
    fn from(value: FieldError) -> Self {
        Self::Error(value)
    }
}

impl From<FieldValidity> for FieldChild {
    fn from(value: FieldValidity) -> Self {
        Self::Validity(value)
    }
}

impl From<NumberFieldRoot> for FieldChild {
    fn from(value: NumberFieldRoot) -> Self {
        Self::NumberField(value)
    }
}

pub enum FieldItemChild {
    Label(FieldLabel),
    Control(FieldControl),
    Description(FieldDescription),
    Error(FieldError),
    Validity(FieldValidity),
    NumberField(NumberFieldRoot),
    Any(AnyElement),
}

impl IntoElement for FieldItemChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Label(label) => label.into_any_element(),
            Self::Control(control) => control.into_any_element(),
            Self::Description(description) => description.into_any_element(),
            Self::Error(error) => error.into_any_element(),
            Self::Validity(validity) => validity.into_any_element(),
            Self::NumberField(number_field) => number_field.into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl From<FieldLabel> for FieldItemChild {
    fn from(value: FieldLabel) -> Self {
        Self::Label(value)
    }
}

impl From<FieldControl> for FieldItemChild {
    fn from(value: FieldControl) -> Self {
        Self::Control(value)
    }
}

impl From<FieldDescription> for FieldItemChild {
    fn from(value: FieldDescription) -> Self {
        Self::Description(value)
    }
}

impl From<FieldError> for FieldItemChild {
    fn from(value: FieldError) -> Self {
        Self::Error(value)
    }
}

impl From<FieldValidity> for FieldItemChild {
    fn from(value: FieldValidity) -> Self {
        Self::Validity(value)
    }
}

impl From<NumberFieldRoot> for FieldItemChild {
    fn from(value: NumberFieldRoot) -> Self {
        Self::NumberField(value)
    }
}

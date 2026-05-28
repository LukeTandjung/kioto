use gpui::{AnyElement, IntoElement};

use crate::{api::GenericChild, checkbox::{CheckboxContext, CheckboxIndicator}};

pub enum CheckboxChild {
    Indicator(CheckboxIndicator),
    Any(AnyElement),
}

impl IntoElement for CheckboxChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Indicator(indicator) => indicator.into_any_element(),
            Self::Any(element) => element,
        }
    }
}

impl GenericChild<CheckboxContext> for CheckboxChild {
    fn add_state_context(self, context: CheckboxContext) -> Self {
        match self {
            Self::Indicator(indicator) => Self::Indicator(indicator.add_state_context(context)),
            Self::Any(element) => Self::Any(element),
        }
    }
}

impl From<CheckboxIndicator> for CheckboxChild {
    fn from(value: CheckboxIndicator) -> Self { Self::Indicator(value) }
}

impl From<AnyElement> for CheckboxChild {
    fn from(value: AnyElement) -> Self { Self::Any(value) }
}

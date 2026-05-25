use gpui::{AnyElement, IntoElement};

use crate::{
    api::GenericChild,
    tabs::{TabsIndicator, TabsList, TabsPanel, TabsProps, TabsState},
    utils::ControlledContext,
};

pub enum TabsChild<T: Clone + Eq + 'static> {
    List(TabsList<T>),
    Panel(TabsPanel<T>),
    Indicator(TabsIndicator),
}

impl<T: Clone + Eq + 'static> IntoElement for TabsChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::List(list) => list.into_any_element(),
            Self::Panel(panel) => panel.into_any_element(),
            Self::Indicator(indicator) => indicator.into_any_element(),
        }
    }
}

impl<T: Clone + Eq + 'static> GenericChild<ControlledContext<TabsState<T>, TabsProps<T>>>
    for TabsChild<T>
{
    fn add_state_context(self, context: ControlledContext<TabsState<T>, TabsProps<T>>) -> Self {
        match self {
            Self::List(list) => Self::List(list.add_state_context(context)),
            Self::Panel(panel) => Self::Panel(panel.add_state_context(context)),
            Self::Indicator(indicator) => Self::Indicator(indicator),
        }
    }
}

impl<T: Clone + Eq + 'static> From<TabsList<T>> for TabsChild<T> {
    fn from(value: TabsList<T>) -> Self {
        Self::List(value)
    }
}

impl<T: Clone + Eq + 'static> From<TabsPanel<T>> for TabsChild<T> {
    fn from(value: TabsPanel<T>) -> Self {
        Self::Panel(value)
    }
}

impl<T: Clone + Eq + 'static> From<TabsIndicator> for TabsChild<T> {
    fn from(value: TabsIndicator) -> Self {
        Self::Indicator(value)
    }
}

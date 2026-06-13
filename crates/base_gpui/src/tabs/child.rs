use gpui::{AnyElement, App, IntoElement, Window};

use crate::{
    api::GenericChild,
    tabs::{TabsContext, TabsIndicator, TabsList, TabsPanel, TabsTab},
};

pub enum TabsChild<T: Clone + Eq + 'static> {
    List(TabsList<T>),
    Panel(TabsPanel<T>),
    Indicator(TabsIndicator<T>),
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

impl<T: Clone + Eq + 'static> GenericChild<TabsContext<T>> for TabsChild<T> {
    fn add_state_context(self, context: TabsContext<T>) -> Self {
        match self {
            Self::List(list) => Self::List(list.add_state_context(context)),
            Self::Panel(panel) => Self::Panel(panel.add_state_context(context)),
            Self::Indicator(indicator) => Self::Indicator(indicator.add_state_context(context)),
        }
    }
}

impl<T: Clone + Eq + 'static> TabsChild<T> {
    pub fn register_runtime(&self, context: &TabsContext<T>, window: &mut Window, cx: &mut App) {
        match self {
            Self::List(list) => list.register_runtime(context, window, cx),
            Self::Panel(_) | Self::Indicator(_) => {}
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

impl<T: Clone + Eq + 'static> From<TabsIndicator<T>> for TabsChild<T> {
    fn from(value: TabsIndicator<T>) -> Self {
        Self::Indicator(value)
    }
}

pub enum TabsListChild<T: Clone + Eq + 'static> {
    Tab(TabsTab<T>),
    Indicator(TabsIndicator<T>),
}

impl<T: Clone + Eq + 'static> TabsListChild<T> {
    pub fn is_tab(&self) -> bool {
        matches!(self, Self::Tab(_))
    }

    pub fn register_runtime(
        &self,
        index: usize,
        context: &TabsContext<T>,
        window: &mut Window,
        cx: &mut App,
    ) {
        match self {
            Self::Tab(tab) => tab.register_runtime(index, context, window, cx),
            Self::Indicator(_) => {}
        }
    }

    pub fn into_any_element_with_context(
        self,
        index: Option<usize>,
        context: Option<TabsContext<T>>,
    ) -> AnyElement {
        match (self, context) {
            (Self::Tab(tab), Some(context)) => tab
                .index(index.expect("tabs tab children must have an index"))
                .add_state_context(context)
                .into_any_element(),
            (Self::Tab(tab), None) => tab
                .index(index.expect("tabs tab children must have an index"))
                .into_any_element(),
            (Self::Indicator(indicator), Some(context)) => {
                indicator.add_state_context(context).into_any_element()
            }
            (Self::Indicator(indicator), None) => indicator.into_any_element(),
        }
    }
}

impl<T: Clone + Eq + 'static> From<TabsTab<T>> for TabsListChild<T> {
    fn from(value: TabsTab<T>) -> Self {
        Self::Tab(value)
    }
}

impl<T: Clone + Eq + 'static> From<TabsIndicator<T>> for TabsListChild<T> {
    fn from(value: TabsIndicator<T>) -> Self {
        Self::Indicator(value)
    }
}

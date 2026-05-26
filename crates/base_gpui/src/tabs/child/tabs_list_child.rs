use gpui::{AnyElement, App, IntoElement};

use crate::{
    api::GenericChild,
    tabs::{TabsContext, TabsIndicator, TabsTab},
};

pub enum TabsListChild<T: Clone + Eq + 'static> {
    Tab(TabsTab<T>),
    Indicator(TabsIndicator<T>),
}

impl<T: Clone + Eq + 'static> TabsListChild<T> {
    pub fn is_tab(&self) -> bool {
        matches!(self, Self::Tab(_))
    }

    pub fn register_runtime(&self, index: usize, context: &TabsContext<T>, cx: &mut App) {
        match self {
            Self::Tab(tab) => tab.register_runtime(index, context, cx),
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

use std::rc::Rc;

use gpui::{App, ClickEvent, Window};

use crate::utils::ControlledContext;

use super::TabsState;

pub struct TabsProps<T: Clone + Eq + 'static> {
    on_value_change: Option<Rc<dyn Fn(Option<&T>, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl<T: Clone + Eq + 'static> Clone for TabsProps<T> {
    fn clone(&self) -> Self {
        Self {
            on_value_change: self.on_value_change.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> TabsProps<T> {
    pub fn new(
        on_value_change: Option<Rc<dyn Fn(Option<&T>, &ClickEvent, &mut Window, &mut App) + 'static>>,
    ) -> Self {
        Self { on_value_change }
    }

    pub fn on_value_change(
        &self,
    ) -> Option<&Rc<dyn Fn(Option<&T>, &ClickEvent, &mut Window, &mut App) + 'static>> {
        self.on_value_change.as_ref()
    }
}

impl<T: Clone + Eq + 'static> ControlledContext<TabsState<T>, TabsProps<T>> {
    pub fn selected_value(&self, cx: &App) -> Option<T> {
        self.get_state(cx)
    }

    pub fn select_value(
        &self,
        value: Option<T>,
        event: &ClickEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        let current = self.selected_value(cx);

        if current == value {
            return;
        }

        if let Some(on_value_change) = self.props().on_value_change() {
            on_value_change(value.as_ref(), event, window, cx);
        }

        self.set_state(value, cx);
    }
}

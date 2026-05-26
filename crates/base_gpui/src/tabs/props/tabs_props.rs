use std::rc::Rc;

use gpui::{App, ClickEvent, Window};

use crate::{
    tabs::{TabsRuntime, TabsState},
    utils::ControlledContext,
};

use super::TabsOrientation;

pub struct TabsProps<T: Clone + Eq + 'static> {
    orientation: TabsOrientation,
    on_value_change: Option<Rc<dyn Fn(Option<&T>, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl<T: Clone + Eq + 'static> Clone for TabsProps<T> {
    fn clone(&self) -> Self {
        Self {
            orientation: self.orientation,
            on_value_change: self.on_value_change.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> TabsProps<T> {
    pub fn new(
        orientation: TabsOrientation,
        on_value_change: Option<Rc<dyn Fn(Option<&T>, &ClickEvent, &mut Window, &mut App) + 'static>>,
    ) -> Self {
        Self {
            orientation,
            on_value_change,
        }
    }

    pub fn orientation(&self) -> TabsOrientation {
        self.orientation
    }

    pub fn on_value_change(
        &self,
    ) -> Option<&Rc<dyn Fn(Option<&T>, &ClickEvent, &mut Window, &mut App) + 'static>> {
        self.on_value_change.as_ref()
    }
}

impl<T: Clone + Eq + 'static> ControlledContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>> {
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
        self.set_state(value, cx, |props, next, cx| {
            props.on_value_change().map(|on_value_change| {
                on_value_change(next, event, window, cx);
            });
        });
    }

    pub fn apply_automatic_fallback(&self, cx: &mut App) {
        if self.is_controlled() {
            return;
        }

        let current = self.get_state(cx);
        let fallback = self.get_runtime(cx, |runtime| {
            if runtime.tabs().is_empty() {
                return current.clone();
            }

            match current.as_ref() {
                Some(value) if runtime.contains_enabled_value(value) => current.clone(),
                _ => runtime.first_enabled_value(),
            }
        });

        if fallback != current {
            self.set_state_silent(fallback, cx);
        }
    }
}

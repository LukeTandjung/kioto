use gpui::{App, ClickEvent, ElementId, Window};

use crate::{
    api::GenericContext,
    tabs::{TabsProps, TabsRuntime, TabsState},
};

pub struct TabsContext<T: Clone + Eq + 'static> {
    inner: GenericContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>,
}

impl<T: Clone + Eq + 'static> Clone for TabsContext<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> TabsContext<T> {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<T>>,
        default: Option<T>,
        props: TabsProps<T>,
        runtime: TabsRuntime<T>,
    ) -> Self {
        Self {
            inner: GenericContext::new(id, cx, window, controlled, default, props, runtime),
        }
    }

    pub fn selected_value(&self, cx: &App) -> Option<T> {
        self.inner.get_state(cx)
    }

    pub fn select_value(
        &self,
        value: Option<T>,
        event: &ClickEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.inner.set_state(value, cx, |props, next, cx| {
            props.on_value_change().map(|on_value_change| {
                on_value_change(next, event, window, cx);
            });
        });
    }

    pub fn apply_automatic_fallback(&self, cx: &mut App) {
        if self.inner.is_controlled() {
            return;
        }

        let current = self.inner.get_state(cx);
        let fallback = self.inner.get_runtime(cx, |runtime| {
            if runtime.tabs().is_empty() {
                return current.clone();
            }

            match current.as_ref() {
                Some(value) if runtime.contains_enabled_value(value) => current.clone(),
                _ => runtime.first_enabled_value(),
            }
        });

        if fallback != current {
            self.inner.set_state_silent(fallback, cx);
        }
    }

    pub fn highlight_tab(&self, index: Option<usize>, cx: &mut App) {
        self.inner.set_runtime(cx, |runtime, _| {
            runtime.set_highlighted_tab_index(index);
        });
    }

    pub fn sync_highlighted_tab_with_selected_value(&self, cx: &mut App) {
        let selected = self.selected_value(cx);

        self.inner.set_runtime(cx, |runtime, _| {
            let highlighted = selected
                .as_ref()
                .and_then(|value| runtime.index_of_enabled_value(value))
                .or_else(|| runtime.first_enabled_index());

            runtime.set_highlighted_tab_index(highlighted);
        });
    }

    pub fn set_runtime(&self, cx: &mut App, set: impl FnOnce(&mut TabsRuntime<T>, &mut App)) {
        self.inner.set_runtime(cx, set);
    }

    pub fn get_runtime<Output>(
        &self,
        cx: &App,
        get: impl FnOnce(&TabsRuntime<T>) -> Output,
    ) -> Output {
        self.inner.get_runtime(cx, get)
    }

    pub fn props(&self) -> &TabsProps<T> {
        self.inner.props()
    }
}

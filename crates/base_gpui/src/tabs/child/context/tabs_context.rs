use gpui::{App, Bounds, ElementId, FocusHandle, Pixels, Window};

use crate::{
    api::GenericContext,
    tabs::{
        TabsActivationDirection, TabsIndicatorRenderState, TabsListRenderState, TabsOrientation,
        TabsPanelRenderState, TabsProps, TabsRootRenderState, TabsRuntime, TabsState,
        TabsTabRenderState,
    },
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
    ) -> Self {
        Self {
            inner: GenericContext::new(
                id,
                cx,
                window,
                controlled,
                default,
                props,
                TabsRuntime::new(),
            ),
        }
    }

    pub fn selected_value(&self, cx: &App) -> Option<T> {
        self.inner.get_state(cx)
    }

    pub fn select_value(&self, value: Option<T>, window: &mut Window, cx: &mut App) {
        let previous = self.selected_value(cx);
        let direction = self.compute_activation_direction(previous.as_ref(), value.as_ref(), cx);
        let is_controlled = self.inner.is_controlled();
        let next_activation_previous_value = match is_controlled {
            true => previous,
            false => value.clone(),
        };

        self.inner.set_runtime(cx, |runtime, _| {
            runtime.set_activation_direction(direction);
            runtime.set_activation_previous_value(next_activation_previous_value);
        });

        self.inner.set_state(value, cx, |props, next, cx| {
            props.on_value_change().map(|on_value_change| {
                on_value_change(next, window, cx);
            });
        });
    }

    pub fn select_highlighted_tab(&self, window: &mut Window, cx: &mut App) {
        let value = self.inner.get_runtime(cx, |runtime| {
            runtime
                .highlighted_tab_index()
                .and_then(|index| runtime.enabled_value_at_index(index).cloned())
        });

        self.select_value(value, window, cx);
    }

    pub fn clear_registered_metadata(&self, cx: &mut App) {
        self.inner.set_runtime(cx, |runtime, _| {
            runtime.clear_tabs();
            runtime.clear_panels();
        });
    }

    pub fn register_tab(&self, value: T, disabled: bool, index: usize, cx: &mut App) {
        self.inner.set_runtime(cx, |runtime, _| {
            runtime.register_tab(value, disabled, index);
        });
    }

    pub fn register_panel(&self, value: T, index: usize, cx: &mut App) {
        self.inner.set_runtime(cx, |runtime, _| {
            runtime.register_panel(value, index);
        });
    }

    pub fn register_tab_bounds(&self, bounds: Vec<(usize, Bounds<Pixels>)>, cx: &mut App) {
        self.inner
            .set_runtime_if_changed(cx, |runtime| runtime.set_tab_bounds(bounds));
    }

    pub fn register_tab_focus_handle(
        &self,
        index: usize,
        focus_handle: FocusHandle,
        cx: &mut App,
    ) {
        self.inner.set_runtime_if_changed(cx, |runtime| {
            runtime.register_tab_focus_handle(index, focus_handle)
        });
    }

    pub fn focus_highlighted_tab(&self, window: &mut Window, cx: &mut App) {
        let focus_handle = self.inner.get_runtime(cx, |runtime| {
            runtime
                .highlighted_tab_index()
                .and_then(|index| runtime.focus_handle_at_index(index))
        });

        if let Some(focus_handle) = focus_handle {
            focus_handle.focus(window, cx);
        }
    }

    pub fn highlight_first_tab(&self, cx: &mut App) {
        let index = self.inner.get_runtime(cx, TabsRuntime::first_enabled_index);

        self.highlight_tab(index, cx);
    }

    pub fn highlight_last_tab(&self, cx: &mut App) {
        let index = self.inner.get_runtime(cx, TabsRuntime::last_enabled_index);

        self.highlight_tab(index, cx);
    }

    pub fn highlight_next_tab(&self, loop_focus: bool, cx: &mut App) {
        let index = self.inner.get_runtime(cx, |runtime| {
            runtime.next_enabled_index(runtime.highlighted_tab_index(), loop_focus)
        });

        if index.is_some() {
            self.highlight_tab(index, cx);
        }
    }

    pub fn highlight_previous_tab(&self, loop_focus: bool, cx: &mut App) {
        let index = self.inner.get_runtime(cx, |runtime| {
            runtime.previous_enabled_index(runtime.highlighted_tab_index(), loop_focus)
        });

        if index.is_some() {
            self.highlight_tab(index, cx);
        }
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
            self.inner.set_runtime(cx, |runtime, _| {
                runtime.set_activation_direction(TabsActivationDirection::None);
                runtime.set_activation_previous_value(fallback.clone());
            });
            self.inner.set_state_silent(fallback, cx);
        }
    }

    pub fn highlight_tab(&self, index: Option<usize>, cx: &mut App) {
        self.inner.set_runtime(cx, |runtime, _| {
            runtime.set_highlighted_tab_index(index);
        });
    }

    pub fn is_tab_highlighted(&self, index: Option<usize>, cx: &App) -> bool {
        self.inner
            .get_runtime(cx, |runtime| runtime.highlighted_tab_index() == index)
    }

    pub fn activation_direction(&self, cx: &App) -> TabsActivationDirection {
        self.inner
            .get_runtime(cx, TabsRuntime::activation_direction)
    }

    pub fn root_render_state(&self, cx: &App) -> TabsRootRenderState {
        TabsRootRenderState::new(self.props().orientation(), self.activation_direction(cx))
    }

    pub fn list_render_state(&self, cx: &App) -> TabsListRenderState {
        TabsListRenderState::new(self.props().orientation(), self.activation_direction(cx))
    }

    pub fn tab_render_state(
        &self,
        value: Option<&T>,
        disabled: bool,
        index: Option<usize>,
        cx: &App,
    ) -> TabsTabRenderState {
        let selected = self.selected_value(cx);
        let active = match (value, selected.as_ref()) {
            (Some(value), Some(selected)) => value == selected,
            _ => false,
        };

        TabsTabRenderState::new(
            active,
            disabled,
            self.is_tab_highlighted(index, cx),
            self.props().orientation(),
        )
    }

    pub fn panel_render_state(&self, value: Option<&T>, cx: &App) -> TabsPanelRenderState {
        let selected = self.selected_value(cx);
        let active = match (value, selected.as_ref()) {
            (Some(value), Some(selected)) => value == selected,
            _ => false,
        };

        TabsPanelRenderState::new(!active, self.props().orientation(), self.activation_direction(cx))
    }

    pub fn indicator_render_state(&self, cx: &App) -> TabsIndicatorRenderState {
        let selected = self.selected_value(cx);

        self.inner.get_runtime(cx, |runtime| {
            TabsIndicatorRenderState::new(
                selected.is_some(),
                runtime.active_tab_position(selected.as_ref()),
                runtime.active_tab_size(selected.as_ref()),
                self.props().orientation(),
                runtime.activation_direction(),
            )
        })
    }

    pub fn sync_activation_direction_with_selected_value(&self, cx: &mut App) {
        let selected = self.selected_value(cx);

        self.inner.set_runtime(cx, |runtime, _| {
            let Some(previous) = runtime.activation_previous_value().cloned() else {
                runtime.set_activation_direction(TabsActivationDirection::None);
                runtime.set_activation_previous_value(selected);
                return;
            };

            if previous == selected {
                return;
            }

            let direction = Self::compute_activation_direction_from_runtime(
                previous.as_ref(),
                selected.as_ref(),
                self.props().orientation(),
                runtime,
            );

            runtime.set_activation_direction(direction);
            runtime.set_activation_previous_value(selected);
        });
    }

    pub fn sync_highlighted_tab_with_selected_value(&self, cx: &mut App) {
        let selected = self.selected_value(cx);

        self.inner.set_runtime(cx, |runtime, _| {
            if runtime.last_synced_selected_value() == Some(&selected) {
                return;
            }

            let highlighted = selected
                .as_ref()
                .and_then(|value| runtime.index_of_enabled_value(value))
                .or_else(|| runtime.first_enabled_index());

            runtime.set_highlighted_tab_index(highlighted);
            runtime.set_last_synced_selected_value(selected);
        });
    }

    pub fn props(&self) -> &TabsProps<T> {
        self.inner.props()
    }

    fn compute_activation_direction(
        &self,
        previous: Option<&T>,
        next: Option<&T>,
        cx: &App,
    ) -> TabsActivationDirection {
        self.inner.get_runtime(cx, |runtime| {
            Self::compute_activation_direction_from_runtime(
                previous,
                next,
                self.props().orientation(),
                runtime,
            )
        })
    }

    fn compute_activation_direction_from_runtime(
        previous: Option<&T>,
        next: Option<&T>,
        orientation: TabsOrientation,
        runtime: &TabsRuntime<T>,
    ) -> TabsActivationDirection {
        let Some(previous) = previous else {
            return TabsActivationDirection::None;
        };
        let Some(next) = next else {
            return TabsActivationDirection::None;
        };

        let Some(previous_index) = runtime.index_of_value(previous) else {
            return TabsActivationDirection::None;
        };
        let Some(next_index) = runtime.index_of_value(next) else {
            return TabsActivationDirection::None;
        };

        match previous_index.cmp(&next_index) {
            std::cmp::Ordering::Less => match orientation {
                TabsOrientation::Horizontal => TabsActivationDirection::Right,
                TabsOrientation::Vertical => TabsActivationDirection::Down,
            },
            std::cmp::Ordering::Greater => match orientation {
                TabsOrientation::Horizontal => TabsActivationDirection::Left,
                TabsOrientation::Vertical => TabsActivationDirection::Up,
            },
            std::cmp::Ordering::Equal => TabsActivationDirection::None,
        }
    }
}

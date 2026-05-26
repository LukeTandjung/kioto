use gpui::{Bounds, FocusHandle, Pixels};

use super::{
    TabsActivationDirection, TabsPanelMetadata, TabsTabMetadata, TabsTabPosition, TabsTabSize,
};

#[derive(Clone)]
pub struct TabsRuntime<T: Clone + Eq + 'static> {
    tabs: Vec<TabsTabMetadata<T>>,
    panels: Vec<TabsPanelMetadata<T>>,
    highlighted_tab_index: Option<usize>,
    last_synced_selected_value: Option<Option<T>>,
    activation_direction: TabsActivationDirection,
    activation_previous_value: Option<Option<T>>,
    tab_bounds: Vec<(usize, Bounds<Pixels>)>,
    tab_focus_handles: Vec<(usize, FocusHandle)>,
}

impl<T: Clone + Eq + 'static> Default for TabsRuntime<T> {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            panels: Vec::new(),
            highlighted_tab_index: None,
            last_synced_selected_value: None,
            activation_direction: TabsActivationDirection::None,
            activation_previous_value: None,
            tab_bounds: Vec::new(),
            tab_focus_handles: Vec::new(),
        }
    }
}

impl<T: Clone + Eq + 'static> TabsRuntime<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_tabs(&mut self) {
        self.tabs.clear();
    }

    pub fn clear_panels(&mut self) {
        self.panels.clear();
    }

    pub fn register_tab(&mut self, value: T, disabled: bool, index: usize) {
        let metadata = TabsTabMetadata::new(value, disabled, index);

        match self.tabs.iter().position(|tab| tab.index() == index) {
            Some(existing_index) => self.tabs[existing_index] = metadata,
            None => self.tabs.push(metadata),
        }

        self.tabs.sort_by_key(TabsTabMetadata::index);
    }

    pub fn register_panel(&mut self, value: T, index: usize) {
        let metadata = TabsPanelMetadata::new(value, index);

        match self.panels.iter().position(|panel| panel.index() == index) {
            Some(existing_index) => self.panels[existing_index] = metadata,
            None => self.panels.push(metadata),
        }

        self.panels.sort_by_key(TabsPanelMetadata::index);
    }

    pub fn tabs(&self) -> &[TabsTabMetadata<T>] {
        &self.tabs
    }

    pub fn panels(&self) -> &[TabsPanelMetadata<T>] {
        &self.panels
    }

    pub fn set_tab_bounds(&mut self, bounds: Vec<(usize, Bounds<Pixels>)>) -> bool {
        if self.tab_bounds == bounds {
            return false;
        }

        self.tab_bounds = bounds;
        true
    }

    pub fn register_tab_focus_handle(&mut self, index: usize, focus_handle: FocusHandle) -> bool {
        match self
            .tab_focus_handles
            .iter()
            .position(|(tab_index, _)| *tab_index == index)
        {
            Some(existing_index) if self.tab_focus_handles[existing_index].1 == focus_handle => false,
            Some(existing_index) => {
                self.tab_focus_handles[existing_index] = (index, focus_handle);
                true
            }
            None => {
                self.tab_focus_handles.push((index, focus_handle));
                self.tab_focus_handles.sort_by_key(|(index, _)| *index);
                true
            }
        }
    }

    pub fn focus_handle_at_index(&self, index: usize) -> Option<FocusHandle> {
        self.tab_focus_handles
            .iter()
            .find(|(tab_index, _)| *tab_index == index)
            .map(|(_, focus_handle)| focus_handle.clone())
    }

    pub fn active_tab_position(&self, selected: Option<&T>) -> Option<TabsTabPosition> {
        let bounds = self.active_tab_bounds(selected)?;

        let origin = self.tab_bounds_origin()?;

        Some(TabsTabPosition::new(
            bounds.left() - origin.0,
            bounds.right() - origin.0,
            bounds.top() - origin.1,
            bounds.bottom() - origin.1,
        ))
    }

    pub fn active_tab_size(&self, selected: Option<&T>) -> Option<TabsTabSize> {
        let bounds = self.active_tab_bounds(selected)?;

        Some(TabsTabSize::new(bounds.size.width, bounds.size.height))
    }

    pub fn highlighted_tab_index(&self) -> Option<usize> {
        self.highlighted_tab_index
    }

    pub fn set_highlighted_tab_index(&mut self, index: Option<usize>) {
        self.highlighted_tab_index = index;
    }

    pub fn last_synced_selected_value(&self) -> Option<&Option<T>> {
        self.last_synced_selected_value.as_ref()
    }

    pub fn set_last_synced_selected_value(&mut self, value: Option<T>) {
        self.last_synced_selected_value = Some(value);
    }

    pub fn activation_direction(&self) -> TabsActivationDirection {
        self.activation_direction
    }

    pub fn set_activation_direction(&mut self, direction: TabsActivationDirection) {
        self.activation_direction = direction;
    }

    pub fn activation_previous_value(&self) -> Option<&Option<T>> {
        self.activation_previous_value.as_ref()
    }

    pub fn set_activation_previous_value(&mut self, value: Option<T>) {
        self.activation_previous_value = Some(value);
    }

    pub fn first_enabled_index(&self) -> Option<usize> {
        self.tabs
            .iter()
            .find(|tab| !tab.disabled())
            .map(TabsTabMetadata::index)
    }

    pub fn last_enabled_index(&self) -> Option<usize> {
        self.tabs
            .iter()
            .rev()
            .find(|tab| !tab.disabled())
            .map(TabsTabMetadata::index)
    }

    pub fn next_enabled_index(&self, current: Option<usize>, loop_focus: bool) -> Option<usize> {
        let current = current.or_else(|| self.first_enabled_index())?;

        self.tabs
            .iter()
            .find(|tab| !tab.disabled() && tab.index() > current)
            .map(TabsTabMetadata::index)
            .or_else(|| loop_focus.then(|| self.first_enabled_index()).flatten())
    }

    pub fn previous_enabled_index(&self, current: Option<usize>, loop_focus: bool) -> Option<usize> {
        let current = current.or_else(|| self.first_enabled_index())?;

        self.tabs
            .iter()
            .rev()
            .find(|tab| !tab.disabled() && tab.index() < current)
            .map(TabsTabMetadata::index)
            .or_else(|| loop_focus.then(|| self.last_enabled_index()).flatten())
    }

    pub fn enabled_value_at_index(&self, index: usize) -> Option<&T> {
        self.tabs
            .iter()
            .find(|tab| !tab.disabled() && tab.index() == index)
            .map(TabsTabMetadata::value)
    }

    pub fn index_of_value(&self, value: &T) -> Option<usize> {
        self.tabs
            .iter()
            .find(|tab| tab.value() == value)
            .map(TabsTabMetadata::index)
    }

    pub fn index_of_enabled_value(&self, value: &T) -> Option<usize> {
        self.tabs
            .iter()
            .find(|tab| !tab.disabled() && tab.value() == value)
            .map(TabsTabMetadata::index)
    }

    pub fn first_enabled_value(&self) -> Option<T> {
        self.tabs
            .iter()
            .find(|tab| !tab.disabled())
            .map(|tab| tab.value().clone())
    }

    pub fn contains_enabled_value(&self, value: &T) -> bool {
        self.tabs
            .iter()
            .any(|tab| !tab.disabled() && tab.value() == value)
    }

    fn active_tab_bounds(&self, selected: Option<&T>) -> Option<Bounds<Pixels>> {
        let selected = selected?;
        let index = self.index_of_value(selected)?;

        self.tab_bounds
            .iter()
            .find(|(tab_index, _)| *tab_index == index)
            .map(|(_, bounds)| *bounds)
    }

    fn tab_bounds_origin(&self) -> Option<(Pixels, Pixels)> {
        let first = self.tab_bounds.first()?.1;

        Some(
            self.tab_bounds
                .iter()
                .fold((first.left(), first.top()), |origin, (_, bounds)| {
                    (origin.0.min(bounds.left()), origin.1.min(bounds.top()))
                }),
        )
    }
}

use super::{TabsActivationDirection, TabsPanelMetadata, TabsTabMetadata};

#[derive(Clone)]
pub struct TabsRuntime<T: Clone + Eq + 'static> {
    tabs: Vec<TabsTabMetadata<T>>,
    panels: Vec<TabsPanelMetadata<T>>,
    highlighted_tab_index: Option<usize>,
    last_synced_selected_value: Option<Option<T>>,
    activation_direction: TabsActivationDirection,
    activation_previous_value: Option<Option<T>>,
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
}

use super::{TabsPanelMetadata, TabsTabMetadata};

#[derive(Clone)]
pub struct TabsRuntime<T: Clone + Eq + 'static> {
    tabs: Vec<TabsTabMetadata<T>>,
    panels: Vec<TabsPanelMetadata<T>>,
    highlighted_tab_index: Option<usize>,
}

impl<T: Clone + Eq + 'static> Default for TabsRuntime<T> {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            panels: Vec::new(),
            highlighted_tab_index: None,
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

    pub fn first_enabled_index(&self) -> Option<usize> {
        self.tabs
            .iter()
            .find(|tab| !tab.disabled())
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

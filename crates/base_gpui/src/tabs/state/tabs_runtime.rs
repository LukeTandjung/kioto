use super::TabsTabMetadata;

#[derive(Clone)]
pub struct TabsRuntime<T: Clone + Eq + 'static> {
    tabs: Vec<TabsTabMetadata<T>>,
}

impl<T: Clone + Eq + 'static> Default for TabsRuntime<T> {
    fn default() -> Self {
        Self { tabs: Vec::new() }
    }
}

impl<T: Clone + Eq + 'static> TabsRuntime<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_tabs(&mut self) {
        self.tabs.clear();
    }

    pub fn register_tab(&mut self, value: T, disabled: bool, index: usize) {
        let metadata = TabsTabMetadata::new(value, disabled, index);

        match self.tabs.iter().position(|tab| tab.index() == index) {
            Some(existing_index) => self.tabs[existing_index] = metadata,
            None => self.tabs.push(metadata),
        }

        self.tabs.sort_by_key(TabsTabMetadata::index);
    }

    pub fn tabs(&self) -> &[TabsTabMetadata<T>] {
        &self.tabs
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

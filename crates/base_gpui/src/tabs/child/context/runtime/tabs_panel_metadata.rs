#[derive(Clone)]
pub struct TabsPanelMetadata<T: Clone + Eq + 'static> {
    value: T,
    index: usize,
}

impl<T: Clone + Eq + 'static> TabsPanelMetadata<T> {
    pub fn new(value: T, index: usize) -> Self {
        Self { value, index }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

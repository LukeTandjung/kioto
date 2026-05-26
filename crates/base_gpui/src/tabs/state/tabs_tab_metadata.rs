#[derive(Clone)]
pub struct TabsTabMetadata<T: Clone + Eq + 'static> {
    value: T,
    disabled: bool,
    index: usize,
}

impl<T: Clone + Eq + 'static> TabsTabMetadata<T> {
    pub fn new(value: T, disabled: bool, index: usize) -> Self {
        Self {
            value,
            disabled,
            index,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

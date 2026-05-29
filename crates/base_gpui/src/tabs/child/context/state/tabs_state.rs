use crate::api::GenericState;

pub struct TabsState<T: Clone + Eq + 'static> {
    value: Option<T>,
}

impl<T: Clone + Eq + 'static> GenericState for TabsState<T> {
    type Value = T;

    fn new(default: Option<Self::Value>) -> Self {
        Self { value: default }
    }

    fn get_value(&self) -> Option<&Self::Value> {
        self.value.as_ref()
    }

    fn set_value(&mut self, value: Option<Self::Value>) {
        self.value = value;
    }
}

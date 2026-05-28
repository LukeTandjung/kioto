use crate::api::GenericState;

pub struct CheckboxState {
    checked: Option<bool>,
}

impl GenericState for CheckboxState {
    type Value = bool;

    fn new(default: Option<Self::Value>) -> Self {
        Self { checked: default }
    }

    fn get_value(&self) -> Option<&Self::Value> {
        self.checked.as_ref()
    }

    fn set_value(&mut self, value: Option<Self::Value>) {
        self.checked = value;
    }
}

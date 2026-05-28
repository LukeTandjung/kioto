use std::rc::Rc;

use gpui::{App, Window};

pub type CheckboxCheckedChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

#[derive(Clone, Default)]
pub struct CheckboxProps {
    indeterminate: bool,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_checked_change: Option<CheckboxCheckedChangeHandler>,
}

impl CheckboxProps {
    pub fn new(
        indeterminate: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        on_checked_change: Option<CheckboxCheckedChangeHandler>,
    ) -> Self {
        Self { indeterminate, disabled, read_only, required, on_checked_change }
    }

    pub fn indeterminate(&self) -> bool { self.indeterminate }
    pub fn disabled(&self) -> bool { self.disabled }
    pub fn read_only(&self) -> bool { self.read_only }
    pub fn required(&self) -> bool { self.required }
    pub fn on_checked_change(&self) -> Option<&CheckboxCheckedChangeHandler> { self.on_checked_change.as_ref() }
}

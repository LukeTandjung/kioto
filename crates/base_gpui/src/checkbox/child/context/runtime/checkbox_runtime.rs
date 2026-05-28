#[derive(Clone, Default)]
pub struct CheckboxRuntime {
    focused: bool,
}

impl CheckboxRuntime {
    pub fn new() -> Self { Self::default() }
    pub fn focused(&self) -> bool { self.focused }
    pub fn set_focused(&mut self, focused: bool) { self.focused = focused; }
}

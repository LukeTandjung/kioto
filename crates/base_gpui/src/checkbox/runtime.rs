use crate::checkbox::{CheckboxIndicatorRenderState, CheckboxProps, CheckboxRootRenderState};

pub struct ToggleOutcome {
    changed: bool,
    checked: bool,
}

impl ToggleOutcome {
    fn new(changed: bool, checked: bool) -> Self {
        Self { changed, checked }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn checked(&self) -> bool {
        self.checked
    }
}

#[derive(Clone, Default)]
pub struct CheckboxRuntime {
    checked: Option<bool>,
    focused: bool,
}

impl CheckboxRuntime {
    pub fn new(checked: Option<bool>) -> Self {
        Self {
            checked,
            ..Self::default()
        }
    }

    pub fn checked_value(&self) -> Option<bool> {
        self.checked
    }

    pub fn sync_checked_from_context(&mut self, checked: Option<bool>) {
        self.checked = checked;
    }

    pub fn checked(&self) -> bool {
        self.checked.unwrap_or(false)
    }

    pub fn toggle(&mut self, disabled: bool, read_only: bool) -> ToggleOutcome {
        let next = !self.checked();

        if disabled || read_only {
            return ToggleOutcome::new(false, self.checked());
        }

        let changed = self.checked != Some(next);
        self.checked = Some(next);

        ToggleOutcome::new(changed, next)
    }

    pub fn sync_focused(&mut self, focused: bool) -> bool {
        if self.focused == focused {
            return false;
        }

        self.focused = focused;
        true
    }

    pub fn root_state(&self, props: &CheckboxProps) -> CheckboxRootRenderState {
        CheckboxRootRenderState::new(
            self.checked(),
            props.disabled(),
            props.read_only(),
            props.required(),
            props.indeterminate(),
            self.focused,
        )
    }

    pub fn indicator_state(
        &self,
        keep_mounted: bool,
        props: &CheckboxProps,
    ) -> CheckboxIndicatorRenderState {
        CheckboxIndicatorRenderState::new(self.root_state(props), keep_mounted)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckboxRuntime;

    #[test]
    fn disabled_toggle_does_not_change_checked_value() {
        let mut runtime = CheckboxRuntime::new(Some(false));

        let outcome = runtime.toggle(true, false);

        assert!(!outcome.changed());
        assert!(!runtime.checked());
    }
}

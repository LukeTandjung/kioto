use crate::tabs::{SelectOutcome, TabsOrientation};

pub trait TabsRuntimeControl<T: Clone + Eq + 'static> {
    fn select_from(
        &mut self,
        current: Option<T>,
        value: Option<T>,
        orientation: TabsOrientation,
        commit: bool,
    ) -> SelectOutcome<T>;
}

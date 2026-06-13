use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::checkbox::{CheckboxProps, CheckboxRuntime};

pub struct CheckboxContext {
    runtime: Entity<CheckboxRuntime>,
    props: Rc<CheckboxProps>,
    controlled: Rc<Option<Option<bool>>>,
}

impl Clone for CheckboxContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl CheckboxContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<bool>>,
        default: Option<bool>,
        props: CheckboxProps,
    ) -> Self {
        let checked = controlled.unwrap_or(default);
        let runtime = window.use_keyed_state(id, cx, |_, _| CheckboxRuntime::new(checked));

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&CheckboxRuntime, &CheckboxProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut CheckboxRuntime) -> Output,
    ) -> Output {
        let controlled = *self.controlled.as_ref();

        self.runtime.update(cx, |runtime, cx| {
            if let Some(checked) = controlled {
                runtime.sync_checked_from_context(checked);
            }

            let output = update(runtime);

            if let Some(checked) = controlled {
                runtime.sync_checked_from_context(checked);
            }

            cx.notify();
            output
        })
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut App) {
        let controlled = *self.controlled.as_ref();
        let props = Rc::clone(&self.props);
        let outcome = self.runtime.update(cx, |runtime, cx| {
            let current = match controlled {
                Some(checked) => checked,
                None => runtime.checked_value(),
            };

            runtime.sync_checked_from_context(current);
            let outcome = runtime.toggle(props.disabled(), props.read_only());

            if controlled.is_some() {
                runtime.sync_checked_from_context(current);
            }

            cx.notify();
            outcome
        });

        if !outcome.changed() {
            return;
        }

        if let Some(on_checked_change) = self.props.on_checked_change() {
            on_checked_change(outcome.checked(), window, cx);
        }
    }
}

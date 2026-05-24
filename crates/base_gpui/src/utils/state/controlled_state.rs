use gpui::{App, ElementId, Entity, Window};

use crate::api::GenericState;

pub struct ControlledState<S: GenericState + 'static> {
    controlled: Option<Option<S::Value>>,
    entity: Entity<S>,
}

impl<S: GenericState + 'static> Clone for ControlledState<S> {
    fn clone(&self) -> Self {
        Self {
            controlled: self.controlled.clone(),
            entity: self.entity.clone(),
        }
    }
}

impl<S: GenericState + 'static> ControlledState<S> {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<S::Value>>,
        default: Option<S::Value>,
    ) -> Self {
        let entity = window.use_keyed_state(id, cx, |_, _| S::new(default));

        Self { controlled, entity }
    }

    pub fn get_state(&self, cx: &App) -> Option<S::Value> {
        self.controlled
            .clone()
            .unwrap_or_else(|| self.entity.read(cx).get_value().cloned())
    }

    pub fn set_state(&self, next: Option<S::Value>, cx: &mut App) {
        if self.controlled.is_none() {
            self.entity.update(cx, |state, cx| {
                state.set_value(next);
                cx.notify();
            });
        }
    }
}

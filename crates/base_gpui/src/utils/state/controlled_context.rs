use gpui::{App, ElementId, Entity, Window};

use crate::api::GenericState;

pub struct ControlledContext<S: GenericState + 'static, D: Clone + 'static> {
    controlled: Option<Option<S::Value>>,
    entity: Entity<S>,
    props: D,
}

impl<S: GenericState + 'static, D: Clone + 'static> Clone for ControlledContext<S, D> {
    fn clone(&self) -> Self {
        Self {
            controlled: self.controlled.clone(),
            entity: self.entity.clone(),
            props: self.props.clone(),
        }
    }
}

impl<S: GenericState + 'static, D: Clone + 'static> ControlledContext<S, D> {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<S::Value>>,
        default: Option<S::Value>,
        props: D,
    ) -> Self {
        let entity = window.use_keyed_state(id, cx, |_, _| S::new(default));

        Self {
            controlled,
            entity,
            props,
        }
    }

    pub fn get_state(&self, cx: &App) -> Option<S::Value> {
        self.controlled
            .clone()
            .unwrap_or_else(|| self.entity.read(cx).get_value().cloned())
    }

    pub fn set_state(
        &self,
        next: Option<S::Value>,
        cx: &mut App,
        notify: impl FnOnce(&D, Option<&S::Value>, &mut App),
    ) {
        if self.get_state(cx) == next {
            return;
        }

        notify(&self.props, next.as_ref(), cx);

        if self.controlled.is_none() {
            self.entity.update(cx, |state, cx| {
                state.set_value(next);
                cx.notify();
            });
        }
    }

    pub fn is_controlled(&self) -> bool {
        self.controlled.is_some()
    }

    pub fn props(&self) -> &D {
        &self.props
    }
}

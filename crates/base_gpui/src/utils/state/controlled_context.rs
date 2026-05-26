use std::sync::Arc;

use gpui::{App, ElementId, Entity, SharedString, Window};

use crate::api::GenericState;

pub struct ControlledContext<S: GenericState + 'static, P: Clone + 'static, R: 'static> {
    controlled: Option<Option<S::Value>>,
    entity: Entity<S>,
    props: P,
    runtime: Entity<R>,
}

impl<S: GenericState + 'static, P: Clone + 'static, R: 'static> Clone
    for ControlledContext<S, P, R>
{
    fn clone(&self) -> Self {
        Self {
            controlled: self.controlled.clone(),
            entity: self.entity.clone(),
            props: self.props.clone(),
            runtime: self.runtime.clone(),
        }
    }
}

impl<S: GenericState + 'static, P: Clone + 'static, R: 'static> ControlledContext<S, P, R> {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<S::Value>>,
        default: Option<S::Value>,
        props: P,
        runtime: R,
    ) -> Self {
        let id = id.into();
        let entity = window.use_keyed_state(id.clone(), cx, |_, _| S::new(default));
        let runtime = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(id), SharedString::from("runtime")),
            cx,
            |_, _| runtime,
        );

        Self {
            controlled,
            entity,
            props,
            runtime,
        }
    }

    pub fn get_state(&self, cx: &App) -> Option<S::Value> {
        match &self.controlled {
            Some(value) => value.clone(),
            None => self.entity.read(cx).get_value().cloned(),
        }
    }

    pub fn set_state(
        &self,
        next: Option<S::Value>,
        cx: &mut App,
        notify: impl FnOnce(&P, Option<&S::Value>, &mut App),
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

    pub fn set_runtime(&self, cx: &mut App, set: impl FnOnce(&mut R, &mut App)) {
        self.runtime.update(cx, |runtime, cx| {
            set(runtime, cx);
            cx.notify();
        });
    }

    pub fn get_runtime<Output>(&self, cx: &App, get: impl FnOnce(&R) -> Output) -> Output {
        get(self.runtime.read(cx))
    }

    pub fn is_controlled(&self) -> bool {
        self.controlled.is_some()
    }

    pub fn props(&self) -> &P {
        &self.props
    }
}

use std::{rc::Rc, sync::Arc};

use gpui::{App, ElementId, Entity, SharedString, Window};

use crate::api::GenericState;

pub struct GenericContext<S: GenericState + 'static, P: Clone + 'static, R: 'static> {
    controlled: Rc<Option<Option<S::Value>>>,
    entity: Entity<S>,
    props: P,
    runtime: Entity<R>,
}

impl<S: GenericState + 'static, P: Clone + 'static, R: 'static> Clone for GenericContext<S, P, R> {
    fn clone(&self) -> Self {
        Self {
            controlled: Rc::clone(&self.controlled),
            entity: self.entity.clone(),
            props: self.props.clone(),
            runtime: self.runtime.clone(),
        }
    }
}

impl<S: GenericState + 'static, P: Clone + 'static, R: 'static> GenericContext<S, P, R> {
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
            controlled: Rc::new(controlled),
            entity,
            props,
            runtime,
        }
    }

    pub fn get_state(&self, cx: &App) -> Option<S::Value> {
        self.read_state(cx, |value| value.cloned())
    }

    pub fn read_state<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(Option<&S::Value>) -> Output,
    ) -> Output {
        match self.controlled.as_ref() {
            Some(value) => read(value.as_ref()),
            None => read(self.entity.read(cx).get_value()),
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
        self.set_state_silent(next, cx);
    }

    pub fn set_state_silent(&self, next: Option<S::Value>, cx: &mut App) {
        if self.controlled.as_ref().is_none() {
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

    pub fn set_runtime_if_changed(&self, cx: &mut App, set: impl FnOnce(&mut R) -> bool) {
        self.runtime.update(cx, |runtime, cx| {
            if set(runtime) {
                cx.notify();
            }
        });
    }

    pub fn get_runtime<Output>(&self, cx: &App, get: impl FnOnce(&R) -> Output) -> Output {
        get(self.runtime.read(cx))
    }

    pub fn is_controlled(&self) -> bool {
        self.controlled.as_ref().is_some()
    }

    pub fn props(&self) -> &P {
        &self.props
    }
}

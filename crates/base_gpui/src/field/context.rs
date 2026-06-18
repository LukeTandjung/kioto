use std::{cell::RefCell, rc::Rc};

use gpui::{App, ElementId, Entity, Window};

use crate::field::{
    FieldControlRegistration, FieldProps, FieldRuntime, FieldValidationMode, FieldValidationResult,
    FieldValidityData, FieldValidityState,
};

thread_local! {
    static FIELD_CONTEXT_STACK: RefCell<Vec<FieldContext>> = const { RefCell::new(Vec::new()) };
}

pub fn current_field_context() -> Option<FieldContext> {
    FIELD_CONTEXT_STACK.with(|stack| stack.borrow().last().cloned())
}

pub fn with_field_context<Output>(context: FieldContext, f: impl FnOnce() -> Output) -> Output {
    struct FieldContextGuard;

    impl Drop for FieldContextGuard {
        fn drop(&mut self) {
            FIELD_CONTEXT_STACK.with(|stack| {
                stack.borrow_mut().pop();
            });
        }
    }

    FIELD_CONTEXT_STACK.with(|stack| stack.borrow_mut().push(context));
    let _guard = FieldContextGuard;

    f()
}

pub struct FieldContext {
    runtime: Entity<FieldRuntime>,
    props: Rc<FieldProps>,
}

impl Clone for FieldContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
        }
    }
}

impl FieldContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        props: FieldProps,
    ) -> Self {
        let runtime = window.use_keyed_state(id, cx, |_, _| FieldRuntime::new());

        Self {
            runtime,
            props: Rc::new(props),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&FieldRuntime, &FieldProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut FieldRuntime) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime);

            cx.notify();
            output
        })
    }

    pub fn begin_registration_pass(&self, cx: &mut App) {
        self.runtime.update(cx, |runtime, _cx| {
            runtime.begin_registration_pass();
        });
    }

    pub fn finish_registration_pass(&self, cx: &mut App) {
        let props = Rc::clone(&self.props);
        self.runtime.update(cx, |runtime, cx| {
            if runtime.finish_registration_pass(props.as_ref()) {
                cx.notify();
            }
        });
    }

    pub fn take_validation_request(&self, cx: &mut App) -> bool {
        self.runtime
            .update(cx, |runtime, _cx| runtime.take_validation_request())
    }

    pub fn take_refresh_request(&self, cx: &mut App) -> bool {
        self.runtime
            .update(cx, |runtime, _cx| runtime.take_refresh_request())
    }

    pub fn register_control(&self, registration: FieldControlRegistration, cx: &mut App) {
        let props = Rc::clone(&self.props);
        self.runtime.update(cx, |runtime, cx| {
            let previous_value = runtime.value();
            let changed = runtime.register_control(registration);
            let value_changed = previous_value != runtime.value();

            if changed {
                cx.notify();
            }

            if value_changed && props.validation_mode() == FieldValidationMode::OnChange {
                runtime.request_validation();
                cx.notify();
            }
        });
    }

    pub fn register_label(&self, cx: &mut App) {
        self.runtime
            .update(cx, |runtime, _cx| runtime.register_label());
    }

    pub fn register_description(&self, cx: &mut App) {
        self.runtime
            .update(cx, |runtime, _cx| runtime.register_description());
    }

    pub fn register_error(&self, cx: &mut App) {
        self.runtime
            .update(cx, |runtime, _cx| runtime.register_error());
    }

    pub fn mark_touched(&self, window: &mut Window, cx: &mut App) {
        let validation_mode = self.props.validation_mode();
        let changed = self.runtime.update(cx, |runtime, cx| {
            let changed = runtime.mark_touched();
            if changed {
                cx.notify();
            }
            changed
        });

        if changed && validation_mode == FieldValidationMode::OnBlur {
            self.validate(window, cx);
        }
    }

    pub fn validate(&self, window: &mut Window, cx: &mut App) {
        let value = self.read(cx, |runtime, _| runtime.value());
        let initial_value = self.read(cx, |runtime, _| runtime.initial_value());
        let result = match self.props.validate() {
            Some(validate) => validate(&value, window, cx),
            None if self.read(cx, |runtime, _| runtime.required()) => {
                if value.filled() {
                    FieldValidationResult::Valid
                } else {
                    FieldValidationResult::Validity(FieldValidityData {
                        state: FieldValidityState::value_missing(),
                        error: "Required".into(),
                        errors: vec!["Required".into()],
                        value: value.clone(),
                        initial_value: initial_value.clone(),
                    })
                }
            }
            None => FieldValidationResult::Validity({
                let mut data = self.read(cx, |runtime, props| runtime.validity_data(props));
                if data.state.valid.is_none() {
                    data = FieldValidationResult::Valid
                        .into_validity_data(value.clone(), initial_value.clone());
                }
                data
            }),
        };
        let validity_data = result.into_validity_data(value, initial_value);

        self.runtime.update(cx, |runtime, cx| {
            if runtime.set_validity_data(validity_data) {
                cx.notify();
            }
        });
    }

    pub fn focus_control(&self, window: &mut Window, cx: &mut App) {
        if let Some(focus_handle) = self.read(cx, |runtime, _| runtime.focus_handle()) {
            focus_handle.focus(window, cx);
        }
    }
}

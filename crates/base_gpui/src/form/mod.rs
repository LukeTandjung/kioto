pub mod actions;
pub mod context;
pub mod layers;
pub mod props;
pub mod render_state;
pub mod runtime;
pub mod value;

#[cfg(test)]
mod tests;

pub use actions::{init, FormSubmitAction, FormValidateAction, FORM_KEY_CONTEXT};
pub use context::{current_form_context, FormContext};
pub use layers::Form;
pub use props::{FormErrors, FormProps, FormSubmitHandler};
pub use render_state::FormRenderState;
pub use runtime::{
    FormFieldRegistration, FormFieldSnapshot, FormFieldValidationHandler, FormRuntime,
    FormSubmissionResult,
};
pub use value::{FormValue, FormValues};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormSubmitReason {
    Programmatic,
    Action,
}

impl Default for FormSubmitReason {
    fn default() -> Self {
        Self::Programmatic
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FormSubmitDetails {
    pub reason: FormSubmitReason,
}

impl FormSubmitDetails {
    pub fn new(reason: FormSubmitReason) -> Self {
        Self { reason }
    }
}

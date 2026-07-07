pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
mod layers;
pub mod otp;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{
    init, OTPFieldBackspace, OTPFieldClear, OTPFieldDelete, OTPFieldFirst, OTPFieldLast,
    OTPFieldNext, OTPFieldPrevious, OTP_FIELD_KEY_CONTEXT,
};
pub use child::OTPFieldChild;
pub use context::OTPFieldContext;
pub use layers::{OTPFieldInput, OTPFieldRoot};
pub use otp::{
    clamp_code_points, code_point_count, filter_validation, normalize_otp_value, remove_at_index,
    replace_at_index, strip_whitespace, OTPFieldNormalizeValueHandler, OTPFieldValidationType,
    OTPNormalizedValue, OTPReplaceOutcome,
};
pub use props::{
    OTPFieldProps, OTPFieldValueChangeHandler, OTPFieldValueCompleteHandler,
    OTPFieldValueInvalidHandler,
};
pub use runtime::{
    OTPFieldChangeDetails, OTPFieldChangeReason, OTPFieldMove, OTPFieldRuntime,
    OTPFieldUpdateOutcome, OTPFieldValueChange,
};
pub use style_state::{OTPFieldInputStyleState, OTPFieldRootStyleState};

// Matches Base UI's `index.parts.ts`: the OTP Field family re-exports Separator.
pub use crate::separator::Separator;

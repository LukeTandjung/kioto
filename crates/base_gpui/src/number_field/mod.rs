pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
mod layers;
pub mod number;
pub mod props;
pub mod render_state;
pub mod runtime;

#[cfg(test)]
mod tests;

pub use actions::{
    init, NumberFieldMax, NumberFieldMin, NumberFieldStepDown, NumberFieldStepDownLarge,
    NumberFieldStepDownSmall, NumberFieldStepUp, NumberFieldStepUpLarge, NumberFieldStepUpSmall,
    NUMBER_FIELD_KEY_CONTEXT,
};
pub use child::{NumberFieldChild, NumberFieldGroupChild};
pub use context::NumberFieldContext;
pub use layers::{
    NumberFieldDecrement, NumberFieldGroup, NumberFieldIncrement, NumberFieldInput,
    NumberFieldRoot, NumberFieldScrubArea, NumberFieldScrubAreaCursor,
};
pub use number::{
    clamp_value, clean_floating_point_noise, format_number, normalize_optional_value,
    option_values_equal, parse_number, snap_to_step_grid, step_value, NumberFieldStep,
    NumberParseError,
};
pub use props::{NumberFieldProps, NumberFieldValueChangeHandler, NumberFieldValueCommitHandler};
pub use render_state::{
    NumberFieldDecrementRenderState, NumberFieldGroupRenderState, NumberFieldIncrementRenderState,
    NumberFieldInputRenderState, NumberFieldRootRenderState, NumberFieldScrubAreaCursorRenderState,
    NumberFieldScrubAreaRenderState,
};
pub use runtime::{
    NumberFieldChangeDetails, NumberFieldChangeReason, NumberFieldCommitDetails,
    NumberFieldCommitReason, NumberFieldRuntime, NumberFieldScrubDirection, NumberFieldStepAmount,
    NumberFieldStepDirection, NumberFieldUpdateOutcome, NumberFieldValueChange,
    NumberFieldValueCommit,
};

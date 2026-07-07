pub mod actions;
pub mod handle;
pub mod layers;

#[cfg(test)]
mod tests;

pub use crate::dialog::{
    DialogBackdrop as AlertDialogBackdrop, DialogClose as AlertDialogClose,
    DialogDescription as AlertDialogDescription, DialogPopup as AlertDialogPopup,
    DialogPortal as AlertDialogPortal, DialogTitle as AlertDialogTitle,
    DialogViewport as AlertDialogViewport,
};
pub use actions::init;
pub use handle::{create_alert_dialog_handle, AlertDialogHandle};
pub use layers::{AlertDialogRoot, AlertDialogTrigger};

use std::rc::Rc;

use gpui::{Context, SharedString, Window};

use crate::primitives::input::runtime::InputRuntime;

pub type InputValueChangeHandler =
    Rc<dyn Fn(SharedString, &mut Window, &mut Context<InputRuntime>) + 'static>;
pub type InputEnterHandler = Rc<dyn Fn(SharedString) + 'static>;
pub type InputBoundaryHandler =
    Rc<dyn Fn(SharedString, &mut Window, &mut Context<InputRuntime>) -> bool + 'static>;

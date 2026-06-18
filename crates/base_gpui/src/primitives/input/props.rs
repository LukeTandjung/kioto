use std::rc::Rc;

use gpui::SharedString;

pub type InputValueChangeHandler = Rc<dyn Fn(SharedString) + 'static>;
pub type InputEnterHandler = Rc<dyn Fn(SharedString) + 'static>;

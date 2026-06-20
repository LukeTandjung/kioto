pub mod checkbox;
pub mod checkbox_group;
pub mod field;
pub mod fieldset;
pub mod form;
pub mod input;
pub mod number_field;
pub mod primitives;
pub mod radio_group;
pub mod switch;
pub mod tabs;
pub mod utils;

pub fn init(cx: &mut gpui::App) {
    checkbox::init(cx);
    checkbox_group::init(cx);
    fieldset::init(cx);
    form::init(cx);
    number_field::init(cx);
    primitives::input::init(cx);
    radio_group::init(cx);
    switch::init(cx);
    tabs::init(cx);
}

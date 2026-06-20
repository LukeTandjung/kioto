pub mod checkbox;
pub mod field;
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
    form::init(cx);
    number_field::init(cx);
    primitives::input::init(cx);
    radio_group::init(cx);
    switch::init(cx);
    tabs::init(cx);
}

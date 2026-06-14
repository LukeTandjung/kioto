pub mod checkbox;
pub mod field;
pub mod radio_group;
pub mod switch;
pub mod tabs;
pub mod utils;

pub fn init(cx: &mut gpui::App) {
    checkbox::init(cx);
    radio_group::init(cx);
    switch::init(cx);
    tabs::init(cx);
}

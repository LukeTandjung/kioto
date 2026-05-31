pub mod api;
pub mod checkbox;
pub mod tabs;

pub fn init(cx: &mut gpui::App) {
    checkbox::init(cx);
    tabs::init(cx);
}

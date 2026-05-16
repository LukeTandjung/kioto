use gpui::{
    AppContext, Application, Context, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, WindowOptions, div, white,
};

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |_window, app| {
            app.new(|_cx| HelloWorld {
                text: SharedString::new_static("World"),
            })
        })
        .unwrap();
    });
}

use editor::Editor;
use gpui::*;
use gpui_component::*;
use gpui_platform::application;

const SAMPLE: &str = r#"fn main() {
    println!("Hello from Kioto's editor spike");
}

// Tiny Helix-ish controls:
//   Esc        normal mode
//   i / a      insert / append
//   h j k l    move
//   v          select mode
//   x          select line
//   y / d / p  copy / delete / paste
"#;

fn main() {
    application().run(move |cx: &mut App| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);
        editor::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| Editor::new(window, cx).with_text(SAMPLE));

                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}

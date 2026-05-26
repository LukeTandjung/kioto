use base_gpui::tabs::{TabsList, TabsPanel, TabsRoot, TabsTab};
use gpui::{
    App, Bounds, Context, IntoElement, Render, Window, WindowBounds, WindowOptions, div, prelude::*,
    px, rgb, size,
};
use gpui_platform::application;

struct TabsTest;

impl Render for TabsTest {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0xf3f4f6))
            .child(
                div()
                    .w(px(320.0))
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(0xd1d5db))
                    .bg(rgb(0xffffff))
                    .p_4()
                    .shadow_lg()
                    .child(
                        TabsRoot::<&'static str>::new()
                            .id("test-tabs")
                            .default_value(None)
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                TabsList::new()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        TabsTab::new()
                                            .id("overview-tab")
                                            .value("overview")
                                            .px_3()
                                            .py_2()
                                            .rounded_md()
                                            .style_with_state(|state, tab| {
                                                if state.active {
                                                    tab.bg(rgb(0xe5e7eb))
                                                } else if state.highlighted {
                                                    tab.bg(rgb(0xf3f4f6))
                                                } else {
                                                    tab
                                                }
                                            })
                                            .child("Overview"),
                                    )
                                    .child(
                                        TabsTab::new()
                                            .id("projects-tab")
                                            .value("projects")
                                            .px_3()
                                            .py_2()
                                            .rounded_md()
                                            .style_with_state(|state, tab| {
                                                if state.active {
                                                    tab.bg(rgb(0xe5e7eb))
                                                } else if state.highlighted {
                                                    tab.bg(rgb(0xf3f4f6))
                                                } else {
                                                    tab
                                                }
                                            })
                                            .child("Projects"),
                                    )
                                    .child(
                                        TabsTab::new()
                                            .id("account-tab")
                                            .value("account")
                                            .px_3()
                                            .py_2()
                                            .rounded_md()
                                            .style_with_state(|state, tab| {
                                                if state.active {
                                                    tab.bg(rgb(0xe5e7eb))
                                                } else if state.highlighted {
                                                    tab.bg(rgb(0xf3f4f6))
                                                } else {
                                                    tab
                                                }
                                            })
                                            .child("Account"),
                                    ),
                            )
                            .child(
                                TabsPanel::new()
                                    .value("overview")
                                    .min_h(px(96.0))
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0xe5e7eb))
                                    .p_4()
                                    .child("Workspace stats and activity."),
                            )
                            .child(
                                TabsPanel::new()
                                    .value("projects")
                                    .min_h(px(96.0))
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0xe5e7eb))
                                    .p_4()
                                    .child("Milestones and deadlines."),
                            )
                            .child(
                                TabsPanel::new()
                                    .value("account")
                                    .min_h(px(96.0))
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0xe5e7eb))
                                    .p_4()
                                    .child("Profile and preferences."),
                            ),
                    ),
            )
    }
}

fn main() {
    application().run(|cx: &mut App| {
        base_gpui::init(cx);

        let bounds = Bounds::centered(None, size(px(500.0), px(360.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TabsTest),
        )
        .expect("failed to open tabs test window");

        cx.activate(true);
    });
}

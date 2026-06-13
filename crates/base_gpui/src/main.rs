use base_gpui::{
    checkbox::{CheckboxIndicator, CheckboxRoot},
    tabs::{TabsIndicator, TabsList, TabsPanel, TabsRoot, TabsTab},
};
use gpui::{
    div, prelude::*, px, rgb, size, App, Bounds, Context, IntoElement, Render, Window,
    WindowBounds, WindowOptions,
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
                    .flex()
                    .flex_col()
                    .gap_5()
                    .child(
                        TabsRoot::<&'static str>::new()
                            .id("test-tabs")
                            .default_value(None)
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                TabsList::new()
                                    .relative()
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
                                    )
                                    .child(
                                        TabsIndicator::new()
                                            .absolute()
                                            .h(px(2.0))
                                            .rounded_full()
                                            .style_with_state(|state, indicator| {
                                                let indicator = match state.active_tab_position {
                                                    Some(position) => indicator
                                                        .left(position.left)
                                                        .top(position.bottom - px(2.0)),
                                                    None => indicator,
                                                };
                                                let indicator = match state.active_tab_size {
                                                    Some(size) => indicator.w(size.width),
                                                    None => indicator,
                                                };

                                                if state.selected {
                                                    indicator.bg(rgb(0x111827))
                                                } else {
                                                    indicator
                                                }
                                            }),
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
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child("Checkbox")
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        CheckboxRoot::new()
                                            .id("example-checkbox")
                                            .default_checked(false)
                                            .size(px(18.0))
                                            .rounded_sm()
                                            .border_1()
                                            .style_with_state(|state, root| {
                                                let root = if state.checked {
                                                    root.bg(rgb(0x111827))
                                                        .border_color(rgb(0x111827))
                                                } else {
                                                    root.bg(rgb(0xffffff))
                                                        .border_color(rgb(0x9ca3af))
                                                };

                                                if state.disabled || state.read_only {
                                                    root.opacity(0.5)
                                                } else {
                                                    root
                                                }
                                            })
                                            .child(
                                                CheckboxIndicator::new()
                                                    .size_full()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_color(rgb(0xffffff))
                                                    .text_size(px(12.0))
                                                    .child("✓"),
                                            ),
                                    )
                                    .child("Click the square to toggle it."),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        CheckboxRoot::new()
                                            .id("checked-checkbox")
                                            .default_checked(true)
                                            .size(px(18.0))
                                            .rounded_sm()
                                            .border_1()
                                            .style_with_state(|state, root| {
                                                if state.checked {
                                                    root.bg(rgb(0x2563eb))
                                                        .border_color(rgb(0x2563eb))
                                                } else {
                                                    root.bg(rgb(0xffffff))
                                                        .border_color(rgb(0x9ca3af))
                                                }
                                            })
                                            .child(
                                                CheckboxIndicator::new()
                                                    .size_full()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_color(rgb(0xffffff))
                                                    .text_size(px(12.0))
                                                    .child("✓"),
                                            ),
                                    )
                                    .child("Starts checked."),
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

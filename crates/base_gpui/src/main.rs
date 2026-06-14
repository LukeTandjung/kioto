use base_gpui::{
    checkbox::{CheckboxIndicator, CheckboxRoot},
    field::{FieldDescription, FieldError, FieldLabel, FieldRoot},
    radio_group::{RadioGroupIndicator, RadioGroupRadio, RadioGroupRoot},
    switch::{SwitchRoot, SwitchThumb},
    tabs::{TabsIndicator, TabsList, TabsPanel, TabsRoot, TabsTab},
    utils::direction::{DirectionProvider, TextDirection},
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
                        div().flex().flex_col().gap_3().child("Switch").child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    SwitchRoot::new()
                                        .id("example-switch")
                                        .default_checked(true)
                                        .w(px(36.0))
                                        .h(px(20.0))
                                        .rounded_full()
                                        .border_1()
                                        .p(px(2.0))
                                        .style_with_state(|state, root| {
                                            let root = if state.checked {
                                                root.bg(rgb(0x111827)).border_color(rgb(0x111827))
                                            } else {
                                                root.bg(rgb(0xffffff)).border_color(rgb(0x9ca3af))
                                            };

                                            if state.focused {
                                                root.shadow_md()
                                            } else {
                                                root
                                            }
                                        })
                                        .child(
                                            SwitchThumb::new()
                                                .size(px(14.0))
                                                .rounded_full()
                                                .style_with_state(|state, thumb| {
                                                    let thumb = if state.root.checked {
                                                        thumb.ml(px(16.0)).bg(rgb(0xffffff))
                                                    } else {
                                                        thumb.bg(rgb(0x111827))
                                                    };

                                                    if state.root.disabled || state.root.read_only {
                                                        thumb.opacity(0.5)
                                                    } else {
                                                        thumb
                                                    }
                                                }),
                                        ),
                                )
                                .child("Notifications"),
                        ),
                    )
                    .child(
                        FieldRoot::new()
                            .id("example-field")
                            .invalid(true)
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                FieldLabel::new()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x374151))
                                    .child("Field-wrapped switch"),
                            )
                            .child_any(
                                SwitchRoot::new()
                                    .id("field-switch")
                                    .default_checked(false)
                                    .w(px(36.0))
                                    .h(px(20.0))
                                    .rounded_full()
                                    .border_1()
                                    .p(px(2.0))
                                    .style_with_state(|state, root| {
                                        if state.checked {
                                            root.bg(rgb(0x111827)).border_color(rgb(0x111827))
                                        } else {
                                            root.bg(rgb(0xffffff)).border_color(rgb(0x9ca3af))
                                        }
                                    })
                                    .child(
                                        SwitchThumb::new()
                                            .size(px(14.0))
                                            .rounded_full()
                                            .style_with_state(|state, thumb| {
                                                if state.root.checked {
                                                    thumb.ml(px(16.0)).bg(rgb(0xffffff))
                                                } else {
                                                    thumb.bg(rgb(0x111827))
                                                }
                                            }),
                                    ),
                            )
                            .child(
                                FieldError::new()
                                    .text_color(rgb(0xdc2626))
                                    .text_size(px(12.0))
                                    .child("This field is marked invalid."),
                            )
                            .child(
                                FieldDescription::new()
                                    .text_color(rgb(0x6b7280))
                                    .text_size(px(12.0))
                                    .child("Labels can focus the registered control."),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child("Radio Group (LTR arrows)")
                            .child(
                                DirectionProvider::new()
                                    .direction(TextDirection::Ltr)
                                    .child(
                                        RadioGroupRoot::<&'static str>::new()
                                            .id("example-radio-group-ltr")
                                            .default_value(Some("standard"))
                                            .flex()
                                            .gap_2()
                                            .child(example_radio("ltr", "standard"))
                                            .child(example_radio("ltr", "express"))
                                            .child(example_radio("ltr", "overnight")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child("Radio Group (RTL arrows)")
                            .child(
                                DirectionProvider::new()
                                    .direction(TextDirection::Rtl)
                                    .child(
                                        RadioGroupRoot::<&'static str>::new()
                                            .id("example-radio-group-rtl")
                                            .default_value(Some("express"))
                                            .flex()
                                            .gap_2()
                                            .child(example_radio("rtl", "standard"))
                                            .child(example_radio("rtl", "express"))
                                            .child(example_radio("rtl", "overnight")),
                                    ),
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

fn example_radio(group: &'static str, value: &'static str) -> RadioGroupRadio<&'static str> {
    RadioGroupRadio::new()
        .id(format!("example-radio-{group}-{value}"))
        .value(value)
        .size(px(22.0))
        .rounded_full()
        .border_1()
        .flex()
        .items_center()
        .justify_center()
        .style_with_state(|state, radio| {
            let radio = if state.checked {
                radio.bg(rgb(0x111827)).border_color(rgb(0x111827))
            } else if state.highlighted {
                radio.bg(rgb(0xf3f4f6)).border_color(rgb(0x6b7280))
            } else {
                radio.bg(rgb(0xffffff)).border_color(rgb(0x9ca3af))
            };

            if state.disabled || state.read_only {
                radio.opacity(0.5)
            } else {
                radio
            }
        })
        .child(
            RadioGroupIndicator::new()
                .size(px(8.0))
                .rounded_full()
                .bg(rgb(0xffffff)),
        )
}

fn main() {
    application().run(|cx: &mut App| {
        base_gpui::init(cx);

        let bounds = Bounds::centered(None, size(px(500.0), px(620.0)), cx);

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

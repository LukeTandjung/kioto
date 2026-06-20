use base_gpui::{
    checkbox::{CheckboxIndicator, CheckboxRoot},
    checkbox_group::CheckboxGroup,
    field::{FieldDescription, FieldError, FieldLabel, FieldRoot},
    fieldset::{FieldsetLegend, FieldsetRoot},
    form::Form,
    input::Input,
    number_field::{
        NumberFieldDecrement, NumberFieldGroup, NumberFieldIncrement, NumberFieldInput,
        NumberFieldRoot,
    },
    radio_group::{RadioGroupIndicator, RadioGroupRadio, RadioGroupRoot},
    select::{
        SelectIcon, SelectItem, SelectItemIndicator, SelectItemText, SelectList, SelectPopup,
        SelectPortal, SelectPositioner, SelectRoot, SelectSeparator, SelectTrigger, SelectValue,
    },
    separator::{Separator, SeparatorOrientation},
    switch::{SwitchRoot, SwitchThumb},
    tabs::{TabsIndicator, TabsList, TabsPanel, TabsRoot, TabsTab},
    utils::direction::{DirectionProvider, TextDirection},
};
use gpui::{
    div, prelude::*, px, rgb, size, App, Bounds, Context, IntoElement, Render, Window,
    WindowBounds, WindowOptions,
};
use gpui_platform::application;

struct ComponentGallery;

impl Render for ComponentGallery {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("component-gallery-scroll")
            .size_full()
            .overflow_y_scroll()
            .bg(rgb(0xf3f4f6))
            .child(
                div()
                    .w_full()
                    .p(px(24.0))
                    .flex()
                    .flex_col()
                    .gap_5()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .text_color(rgb(0x111827))
                                    .child("base_gpui component gallery"),
                            )
                            .child(
                                div().text_size(px(13.0)).text_color(rgb(0x6b7280)).child(
                                    "A wrapped grid of the current ported Base UI components.",
                                ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .items_start()
                            .gap_3()
                            .child(component_card(
                                "Tabs",
                                "Keyboard-selectable tab list with indicator and panels.",
                                tabs_demo(),
                            ))
                            .child(component_card(
                                "Switch",
                                "Toggle state, focus styling, thumb state, and callbacks.",
                                switch_demo(),
                            ))
                            .child(component_card(
                                "Field + Switch",
                                "Field label, error, description, and registered control focus.",
                                field_switch_demo(),
                            ))
                            .child(component_card(
                                "Input",
                                "Public Field-aware text input using Input::new().",
                                plain_input_demo(),
                            ))
                            .child(component_card(
                                "Field + Input",
                                "Required validation and Field-derived Input styling.",
                                input_field_demo(),
                            ))
                            .child(component_card(
                                "Form",
                                "Form validation-mode inheritance and named Field registration.",
                                form_demo(),
                            ))
                            .child(component_card(
                                "Fieldset",
                                "Grouped fields with legend state and disabled propagation.",
                                fieldset_demo(),
                            ))
                            .child(component_card(
                                "Separator",
                                "Shared horizontal and vertical visual dividers.",
                                separator_demo(),
                            ))
                            .child(component_card(
                                "Select",
                                "Trigger, value, popup/list, item labels, and indicator state.",
                                select_demo(),
                            ))
                            .child(component_card(
                                "Field + Select",
                                "Required Field registration with a serialized Select value.",
                                field_select_demo(),
                            ))
                            .child(component_card(
                                "Multiple Select",
                                "Ordered multi-value toggling with item indicators.",
                                multiple_select_demo(),
                            ))
                            .child(component_card(
                                "Number Field",
                                "Text editing, stepping, min/max, and formatted numeric value.",
                                number_field_demo(),
                            ))
                            .child(component_card(
                                "Field + Number Field",
                                "Required Field validation with a numeric control.",
                                field_number_field_demo(),
                            ))
                            .child(component_card(
                                "Radio Group LTR",
                                "Direction-aware arrow key navigation in LTR mode.",
                                radio_group_demo("ltr", TextDirection::Ltr, "standard"),
                            ))
                            .child(component_card(
                                "Radio Group RTL",
                                "Direction-aware arrow key navigation in RTL mode.",
                                radio_group_demo("rtl", TextDirection::Rtl, "express"),
                            ))
                            .child(component_card(
                                "Checkbox",
                                "Unchecked and checked states with indicator rendering.",
                                checkbox_demo(),
                            ))
                            .child(component_card(
                                "Checkbox Group",
                                "Shared selected values, parent state, and disabled propagation.",
                                checkbox_group_demo(),
                            )),
                    ),
            )
    }
}

fn component_card(
    title: &'static str,
    description: &'static str,
    content: impl IntoElement,
) -> impl IntoElement {
    div()
        .w(px(300.0))
        .min_h(px(170.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .bg(rgb(0xffffff))
        .p_4()
        .shadow_lg()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_size(px(15.0))
                        .text_color(rgb(0x111827))
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0x6b7280))
                        .child(description),
                ),
        )
        .child(content)
}

fn separator_demo() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            Separator::new()
                .horizontal()
                .bg(rgb(0xd1d5db))
                .style_with_state(|state, separator| match state.orientation {
                    SeparatorOrientation::Horizontal => separator.w_full().h(px(1.0)),
                    SeparatorOrientation::Vertical => separator.w(px(1.0)).h(px(20.0)),
                }),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .text_size(px(13.0))
                .text_color(rgb(0x374151))
                .child("Home")
                .child("Pricing")
                .child("Blog")
                .child(
                    Separator::new()
                        .vertical()
                        .bg(rgb(0x9ca3af))
                        .style_with_state(|state, separator| match state.orientation {
                            SeparatorOrientation::Horizontal => separator.w_full().h(px(1.0)),
                            SeparatorOrientation::Vertical => separator.w(px(1.0)).h(px(20.0)),
                        }),
                )
                .child("Log in")
                .child("Sign up"),
        )
}

fn select_demo() -> impl IntoElement {
    SelectRoot::<&'static str>::new()
        .id("gallery-select")
        .default_value(Some("apple"))
        .item_to_string_value(|value| (*value).into())
        .flex()
        .flex_col()
        .gap_2()
        .child(select_trigger())
        .child(
            SelectPortal::<&'static str>::new().child(
                SelectPositioner::new().side_offset(px(4.0)).child(
                    SelectPopup::new()
                        .w(px(220.0))
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(0xd1d5db))
                        .bg(rgb(0xffffff))
                        .shadow_lg()
                        .p_1()
                        .child(
                            SelectList::new()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(select_item("apple", "Apple"))
                                .child(select_item("banana", "Banana"))
                                .child(
                                    SelectSeparator::new()
                                        .horizontal()
                                        .my_1()
                                        .h(px(1.0))
                                        .bg(rgb(0xe5e7eb)),
                                )
                                .child(select_item("orange", "Orange")),
                        ),
                ),
            ),
        )
}

fn field_select_demo() -> impl IntoElement {
    FieldRoot::new()
        .id("gallery-field-select")
        .flex()
        .flex_col()
        .gap_2()
        .child(
            FieldLabel::new()
                .text_size(px(13.0))
                .child("Favorite fruit"),
        )
        .child_any(
            SelectRoot::<&'static str>::new()
                .id("gallery-field-select-control")
                .name("fruit")
                .required(true)
                .default_value(None)
                .item_to_string_value(|value| (*value).into())
                .child(select_trigger())
                .child(
                    SelectPortal::<&'static str>::new().child(
                        SelectPositioner::new().side_offset(px(4.0)).child(
                            SelectPopup::new()
                                .w(px(220.0))
                                .rounded_md()
                                .border_1()
                                .border_color(rgb(0xd1d5db))
                                .bg(rgb(0xffffff))
                                .shadow_lg()
                                .p_1()
                                .child(
                                    SelectList::new()
                                        .flex()
                                        .flex_col()
                                        .gap_1()
                                        .child(select_item("apple", "Apple"))
                                        .child(select_item("banana", "Banana"))
                                        .child(select_item("orange", "Orange")),
                                ),
                        ),
                    ),
                ),
        )
        .child(
            FieldDescription::new()
                .text_size(px(12.0))
                .child("Pick one value."),
        )
        .child(
            FieldError::new()
                .text_size(px(12.0))
                .text_color(rgb(0xb91c1c)),
        )
}

fn multiple_select_demo() -> impl IntoElement {
    SelectRoot::<&'static str>::new()
        .id("gallery-multiple-select")
        .multiple(true)
        .default_values(vec!["apple", "orange"])
        .item_to_string_value(|value| (*value).into())
        .flex()
        .flex_col()
        .gap_2()
        .child(select_trigger())
        .child(
            SelectPortal::<&'static str>::new().child(
                SelectPositioner::new().side_offset(px(4.0)).child(
                    SelectPopup::new()
                        .w(px(220.0))
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(0xd1d5db))
                        .bg(rgb(0xffffff))
                        .shadow_lg()
                        .p_1()
                        .child(
                            SelectList::new()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(select_item("apple", "Apple"))
                                .child(select_item("banana", "Banana"))
                                .child(select_item("orange", "Orange")),
                        ),
                ),
            ),
        )
}

fn select_trigger() -> SelectTrigger<&'static str> {
    SelectTrigger::new()
        .w_full()
        .h(px(34.0))
        .px_2()
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .bg(rgb(0xffffff))
        .flex()
        .items_center()
        .justify_between()
        .child(
            SelectValue::new()
                .placeholder("Select a fruit")
                .text_size(px(13.0))
                .text_color(rgb(0x111827)),
        )
        .child(
            SelectIcon::new()
                .text_size(px(12.0))
                .text_color(rgb(0x6b7280)),
        )
}

fn select_item(value: &'static str, label: &'static str) -> SelectItem<&'static str> {
    SelectItem::new()
        .id(format!("gallery-select-item-{value}"))
        .value(value)
        .label(label)
        .px_2()
        .py_1()
        .rounded_sm()
        .flex()
        .items_center()
        .gap_2()
        .style_with_state(|state, item| {
            if state.highlighted {
                item.bg(rgb(0xf3f4f6))
            } else {
                item
            }
        })
        .child(
            SelectItemIndicator::new()
                .keep_mounted(true)
                .w(px(14.0))
                .text_size(px(12.0))
                .style_with_state(|state, indicator| {
                    if state.selected {
                        indicator.text_color(rgb(0x2563eb))
                    } else {
                        indicator.text_color(rgb(0xffffff))
                    }
                }),
        )
        .child(SelectItemText::new().text(label).text_size(px(13.0)))
}

fn tabs_demo() -> impl IntoElement {
    TabsRoot::<&'static str>::new()
        .id("gallery-tabs")
        .default_value(None)
        .flex()
        .flex_col()
        .gap_3()
        .child(
            TabsList::new()
                .relative()
                .flex()
                .gap_2()
                .child(gallery_tab("overview-tab", "overview", "Overview"))
                .child(gallery_tab("projects-tab", "projects", "Projects"))
                .child(gallery_tab("account-tab", "account", "Account"))
                .child(
                    TabsIndicator::new()
                        .absolute()
                        .h(px(2.0))
                        .rounded_full()
                        .style_with_state(|state, indicator| {
                            let indicator = match state.active_tab_position {
                                Some(position) => {
                                    indicator.left(position.left).top(position.bottom - px(2.0))
                                }
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
        .child(gallery_panel("overview", "Workspace stats and activity."))
        .child(gallery_panel("projects", "Milestones and deadlines."))
        .child(gallery_panel("account", "Profile and preferences."))
}

fn gallery_tab(
    id: &'static str,
    value: &'static str,
    label: &'static str,
) -> TabsTab<&'static str> {
    TabsTab::new()
        .id(id)
        .value(value)
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
        .child(label)
}

fn gallery_panel(value: &'static str, text: &'static str) -> TabsPanel<&'static str> {
    TabsPanel::new()
        .value(value)
        .min_h(px(80.0))
        .rounded_md()
        .border_1()
        .border_color(rgb(0xe5e7eb))
        .p_3()
        .child(text)
}

fn switch_demo() -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(gallery_switch("example-switch", true))
        .child("Notifications")
}

fn gallery_switch(id: &'static str, default_checked: bool) -> SwitchRoot {
    SwitchRoot::new()
        .id(id)
        .default_checked(default_checked)
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
                root.shadow_lg()
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
        )
}

fn field_switch_demo() -> impl IntoElement {
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
        .child_any(gallery_switch("field-switch", false))
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
        )
}

fn plain_input_demo() -> impl IntoElement {
    Input::new()
        .id("example-plain-input")
        .name("plain")
        .default_value("Hello GPUI")
        .placeholder("Type here")
        .w_full()
        .h(px(32.0))
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .px_2()
        .style_with_state(|state, input| {
            if state.focused {
                input.border_color(rgb(0x2563eb))
            } else {
                input
            }
        })
}

fn input_field_demo() -> impl IntoElement {
    FieldRoot::new()
        .id("example-input-field")
        .validation_mode(base_gpui::field::FieldValidationMode::OnBlur)
        .flex()
        .flex_col()
        .gap_2()
        .child(
            FieldLabel::new()
                .text_size(px(13.0))
                .text_color(rgb(0x374151))
                .child("Email"),
        )
        .child(
            Input::new()
                .id("example-email-input")
                .name("email")
                .required(true)
                .placeholder("hello@example.com")
                .w_full()
                .h(px(32.0))
                .rounded_md()
                .border_1()
                .px_2()
                .style_with_state(|state, input| {
                    let input = if state.invalid {
                        input.border_color(rgb(0xdc2626))
                    } else if state.focused {
                        input.border_color(rgb(0x2563eb))
                    } else {
                        input.border_color(rgb(0xd1d5db))
                    };

                    if state.disabled || state.read_only {
                        input.opacity(0.5)
                    } else {
                        input
                    }
                }),
        )
        .child(
            FieldError::new()
                .text_color(rgb(0xdc2626))
                .text_size(px(12.0))
                .child("Email is required."),
        )
        .child(
            FieldDescription::new()
                .text_color(rgb(0x6b7280))
                .text_size(px(12.0))
                .child("The public Input component composes FieldControl."),
        )
}

fn form_demo() -> impl IntoElement {
    Form::new()
        .id("example-form")
        .validation_mode(base_gpui::field::FieldValidationMode::OnChange)
        .flex()
        .flex_col()
        .gap_2()
        .child(
            FieldRoot::new()
                .id("example-form-email-field")
                .name("email")
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    FieldLabel::new()
                        .text_size(px(13.0))
                        .text_color(rgb(0x374151))
                        .child("Form email"),
                )
                .child(
                    Input::new()
                        .id("example-form-email-input")
                        .name("email")
                        .required(true)
                        .placeholder("type to validate")
                        .w_full()
                        .h(px(32.0))
                        .rounded_md()
                        .border_1()
                        .px_2()
                        .style_with_state(|state, input| {
                            if state.invalid {
                                input.border_color(rgb(0xdc2626))
                            } else if state.focused {
                                input.border_color(rgb(0x2563eb))
                            } else {
                                input.border_color(rgb(0xd1d5db))
                            }
                        }),
                )
                .child(
                    FieldError::new()
                        .text_color(rgb(0xdc2626))
                        .text_size(px(12.0))
                        .child("Email is required."),
                ),
        )
        .child(
            div()
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .px_3()
                .py_2()
                .text_size(px(12.0))
                .text_color(rgb(0x6b7280))
                .child("Submit is currently exposed as a GPUI Form context/action."),
        )
}

fn fieldset_demo() -> impl IntoElement {
    FieldsetRoot::new()
        .id("example-fieldset")
        .flex()
        .flex_col()
        .gap_2()
        .style_with_state(|state, root| {
            if state.disabled {
                root.opacity(0.5)
            } else {
                root
            }
        })
        .child(
            FieldsetLegend::new()
                .text_size(px(13.0))
                .text_color(rgb(0x111827))
                .style_with_state(|state, legend| {
                    if state.disabled {
                        legend.text_color(rgb(0x9ca3af))
                    } else {
                        legend
                    }
                })
                .child("Billing details"),
        )
        .child_any(fieldset_text_field(
            "example-fieldset-company",
            "Company",
            "Acme Inc.",
        ))
        .child_any(fieldset_text_field(
            "example-fieldset-tax-id",
            "Tax ID",
            "Enter fiscal number",
        ))
}

fn fieldset_text_field(
    id_prefix: &'static str,
    label: &'static str,
    placeholder: &'static str,
) -> impl IntoElement {
    FieldRoot::new()
        .id(id_prefix)
        .flex()
        .flex_col()
        .gap_1()
        .child(
            FieldLabel::new()
                .text_size(px(12.0))
                .text_color(rgb(0x374151))
                .child(label),
        )
        .child(
            Input::new()
                .id(format!("{id_prefix}-input"))
                .placeholder(placeholder)
                .w_full()
                .h(px(30.0))
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .px_2()
                .style_with_state(|state, input| {
                    if state.disabled {
                        input.bg(rgb(0xf3f4f6)).opacity(0.6)
                    } else if state.focused {
                        input.border_color(rgb(0x2563eb))
                    } else {
                        input
                    }
                }),
        )
}

fn number_field_demo() -> impl IntoElement {
    NumberFieldRoot::new()
        .id("example-number-field")
        .default_value(Some(2.0))
        .min(0.0)
        .max(10.0)
        .step(0.5)
        .flex()
        .flex_col()
        .gap_2()
        .child(number_field_group("example-number-field", false))
}

fn field_number_field_demo() -> impl IntoElement {
    FieldRoot::new()
        .id("example-number-field-field")
        .validation_mode(base_gpui::field::FieldValidationMode::OnBlur)
        .flex()
        .flex_col()
        .gap_2()
        .child(
            FieldLabel::new()
                .text_size(px(13.0))
                .text_color(rgb(0x374151))
                .child("Quantity"),
        )
        .child(
            NumberFieldRoot::new()
                .id("example-field-number-field")
                .name("quantity")
                .required(true)
                .min(0.0)
                .max(99.0)
                .default_value(None)
                .child(number_field_group("example-field-number-field", true)),
        )
        .child(
            FieldError::new()
                .text_color(rgb(0xdc2626))
                .text_size(px(12.0))
                .child("Quantity is required."),
        )
}

fn number_field_group(id_prefix: &'static str, show_invalid: bool) -> NumberFieldGroup {
    NumberFieldGroup::new()
        .flex()
        .items_center()
        .gap_1()
        .child(
            NumberFieldDecrement::new()
                .id(format!("{id_prefix}-decrement"))
                .size(px(28.0))
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .flex()
                .items_center()
                .justify_center()
                .style_with_state(|state, decrement| {
                    if state.can_decrement {
                        decrement.bg(rgb(0xffffff))
                    } else {
                        decrement.bg(rgb(0xf3f4f6)).opacity(0.5)
                    }
                })
                .child("−"),
        )
        .child(
            NumberFieldInput::new()
                .w(px(120.0))
                .h(px(32.0))
                .rounded_md()
                .border_1()
                .px_2()
                .style_with_state(move |state, input| {
                    let input = if show_invalid && state.root.invalid {
                        input.border_color(rgb(0xdc2626))
                    } else if state.root.focused {
                        input.border_color(rgb(0x2563eb))
                    } else {
                        input.border_color(rgb(0xd1d5db))
                    };

                    if state.root.disabled || state.root.read_only {
                        input.opacity(0.5)
                    } else {
                        input
                    }
                }),
        )
        .child(
            NumberFieldIncrement::new()
                .id(format!("{id_prefix}-increment"))
                .size(px(28.0))
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .flex()
                .items_center()
                .justify_center()
                .style_with_state(|state, increment| {
                    if state.can_increment {
                        increment.bg(rgb(0xffffff))
                    } else {
                        increment.bg(rgb(0xf3f4f6)).opacity(0.5)
                    }
                })
                .child("+"),
        )
}

fn radio_group_demo(
    group: &'static str,
    direction: TextDirection,
    default_value: &'static str,
) -> impl IntoElement {
    DirectionProvider::new().direction(direction).child(
        RadioGroupRoot::<&'static str>::new()
            .id(format!("example-radio-group-{group}"))
            .default_value(Some(default_value))
            .flex()
            .gap_2()
            .child(example_radio(group, "standard"))
            .child(example_radio(group, "express"))
            .child(example_radio(group, "overnight")),
    )
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

fn checkbox_demo() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(gallery_checkbox("example-checkbox", false, 0x111827))
                .child("Click the square to toggle it."),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(gallery_checkbox("checked-checkbox", true, 0x2563eb))
                .child("Starts checked."),
        )
}

fn checkbox_group_demo() -> impl IntoElement {
    CheckboxGroup::new()
        .id("example-checkbox-group")
        .default_value(["fuji"])
        .all_values(["fuji", "gala", "granny"])
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    gallery_checkbox("example-checkbox-group-parent", false, 0x111827).parent(true),
                )
                .child("All apples"),
        )
        .child(checkbox_group_row("Fuji", "fuji"))
        .child(checkbox_group_row("Gala", "gala"))
        .child(checkbox_group_row("Granny Smith", "granny"))
}

fn checkbox_group_row(label: &'static str, value: &'static str) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            gallery_checkbox("example-checkbox-group-item", false, 0x2563eb)
                .id(format!("example-checkbox-group-{value}"))
                .value(value),
        )
        .child(label)
}

fn gallery_checkbox(id: &'static str, default_checked: bool, checked_color: u32) -> CheckboxRoot {
    CheckboxRoot::new()
        .id(id)
        .default_checked(default_checked)
        .size(px(18.0))
        .rounded_sm()
        .border_1()
        .style_with_state(move |state, root| {
            let root = if state.checked {
                root.bg(rgb(checked_color)).border_color(rgb(checked_color))
            } else {
                root.bg(rgb(0xffffff)).border_color(rgb(0x9ca3af))
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
        )
}

fn main() {
    application().run(|cx: &mut App| {
        base_gpui::init(cx);

        let bounds = Bounds::centered(None, size(px(1040.0), px(760.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ComponentGallery),
        )
        .expect("failed to open component gallery window");

        cx.activate(true);
    });
}

#[cfg(test)]
mod tests {
    use gpui::{px, size, TestAppContext};

    use super::ComponentGallery;

    #[gpui::test]
    fn component_gallery_renders_without_panics(cx: &mut TestAppContext) {
        cx.update(base_gpui::init);
        cx.open_window(size(px(1040.0), px(760.0)), |_, _| ComponentGallery);
        cx.run_until_parked();
    }
}

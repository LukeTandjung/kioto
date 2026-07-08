use std::time::Duration;

use base_gpui::{
    accordion::{AccordionHeader, AccordionItem, AccordionPanel, AccordionRoot, AccordionTrigger},
    alert_dialog::{
        AlertDialogBackdrop, AlertDialogClose, AlertDialogDescription, AlertDialogPopup,
        AlertDialogPortal, AlertDialogRoot, AlertDialogTitle, AlertDialogTrigger,
        AlertDialogViewport,
    },
    autocomplete::{
        AutocompleteClear, AutocompleteEmpty, AutocompleteInput, AutocompleteInputGroup,
        AutocompleteItem, AutocompleteList, AutocompleteMode, AutocompletePopup,
        AutocompletePortal, AutocompletePositioner, AutocompleteRoot, AutocompleteValue,
    },
    avatar::{AvatarFallback, AvatarImage, AvatarRoot},
    button::ButtonRoot,
    checkbox::{CheckboxIndicator, CheckboxRoot},
    checkbox_group::CheckboxGroup,
    collapsible::{CollapsiblePanel, CollapsibleRoot, CollapsibleTrigger},
    combobox::{
        ComboboxChips, ComboboxClear, ComboboxEmpty, ComboboxIcon, ComboboxInput,
        ComboboxInputGroup, ComboboxItem, ComboboxItemIndicator, ComboboxLabel, ComboboxList,
        ComboboxPopup, ComboboxPortal, ComboboxPositioner, ComboboxRoot, ComboboxTrigger,
    },
    context_menu::{
        ContextMenuCheckboxItem, ContextMenuCheckboxItemIndicator, ContextMenuItem,
        ContextMenuPortal, ContextMenuPositioner, ContextMenuRoot, ContextMenuSeparator,
        ContextMenuSubmenuRoot, ContextMenuSubmenuTrigger, ContextMenuTrigger,
    },
    dialog::{
        DialogBackdrop, DialogClose, DialogDescription, DialogPopup, DialogPortal, DialogRoot,
        DialogTitle, DialogTrigger, DialogViewport,
    },
    drawer::{
        DrawerBackdrop, DrawerClose, DrawerContent, DrawerPopup, DrawerPortal, DrawerRoot,
        DrawerSnapPoint, DrawerTitle, DrawerTrigger, DrawerViewport,
    },
    field::{FieldDescription, FieldError, FieldLabel, FieldRoot},
    fieldset::{FieldsetLegend, FieldsetRoot},
    form::Form,
    input::Input,
    menu::{
        MenuCheckboxItem, MenuCheckboxItemIndicator, MenuGroup, MenuGroupLabel, MenuItem,
        MenuPopup, MenuPortal, MenuPositioner, MenuRadioGroup, MenuRadioItem,
        MenuRadioItemIndicator, MenuRoot, MenuSeparator, MenuSubmenuRoot, MenuSubmenuTrigger,
        MenuTrigger,
    },
    menubar::Menubar,
    meter::{MeterIndicator, MeterLabel, MeterRoot, MeterTrack, MeterValue},
    navigation_menu::{
        NavigationMenuArrow, NavigationMenuContent, NavigationMenuIcon, NavigationMenuItem,
        NavigationMenuLink, NavigationMenuList, NavigationMenuPopup, NavigationMenuPortal,
        NavigationMenuPositioner, NavigationMenuRoot, NavigationMenuTrigger,
        NavigationMenuViewport,
    },
    number_field::{
        NumberFieldDecrement, NumberFieldGroup, NumberFieldIncrement, NumberFieldInput,
        NumberFieldRoot,
    },
    otp_field::{OTPFieldInput, OTPFieldRoot},
    popover::{
        PopoverArrow, PopoverClose, PopoverDescription, PopoverPopup, PopoverPortal,
        PopoverPositioner, PopoverRoot, PopoverTitle, PopoverTrigger,
    },
    preview_card::{
        create_preview_card_handle, PreviewCardPopup, PreviewCardPortal, PreviewCardPositioner,
        PreviewCardRoot, PreviewCardTrigger, PreviewCardViewport,
    },
    primitives::{
        scrollbar, scrollbar_vertical, ScrollbarAxis, ScrollbarStyle, ScrollbarVisibility,
    },
    progress::{ProgressIndicator, ProgressLabel, ProgressRoot, ProgressTrack, ProgressValue},
    radio_group::{RadioGroupIndicator, RadioGroupRadio, RadioGroupRoot},
    scroll_area::{
        ScrollAreaContent, ScrollAreaCorner, ScrollAreaOrientation, ScrollAreaRoot,
        ScrollAreaScrollbar, ScrollAreaScrollbarStyleState, ScrollAreaThumb, ScrollAreaViewport,
    },
    select::{
        SelectIcon, SelectItem, SelectItemIndicator, SelectItemText, SelectList, SelectPopup,
        SelectPortal, SelectPositioner, SelectRoot, SelectSeparator, SelectTrigger, SelectValue,
    },
    separator::{Separator, SeparatorOrientation},
    slider::{
        SliderControl, SliderIndicator, SliderLabel, SliderRoot, SliderThumb, SliderTrack,
        SliderValue, SliderValues,
    },
    switch::{SwitchRoot, SwitchThumb},
    tabs::{TabsIndicator, TabsList, TabsOrientation, TabsPanel, TabsRoot, TabsTab},
    toast::{
        create_toast_manager, ToastClose, ToastDescription, ToastId, ToastManager, ToastOptions,
        ToastPortal, ToastPromiseOptions, ToastProvider, ToastRoot, ToastTitle, ToastViewport,
    },
    toggle::Toggle,
    toggle_group::ToggleGroup,
    toolbar::{
        ToolbarButton, ToolbarGroup, ToolbarInput, ToolbarLink, ToolbarRoot, ToolbarSeparator,
    },
    tooltip::{
        TooltipPopup, TooltipPortal, TooltipPositioner, TooltipProvider, TooltipRoot,
        TooltipTrigger, TooltipViewport,
    },
    utils::direction::{DirectionProvider, TextDirection},
};
use gpui::{
    div, hsla, prelude::*, px, rgb, size, uniform_list, AnyElement, App, Bounds, Context, Div,
    IntoElement, Pixels, Render, ScrollHandle, UniformListScrollHandle, Window, WindowBounds,
    WindowOptions,
};
use gpui_platform::application;

struct ComponentGallery {
    scrollbar_vertical_handle: ScrollHandle,
    scrollbar_both_handle: ScrollHandle,
    scrollbar_list_handle: UniformListScrollHandle,
}

impl ComponentGallery {
    fn new() -> Self {
        Self {
            scrollbar_vertical_handle: ScrollHandle::new(),
            scrollbar_both_handle: ScrollHandle::new(),
            scrollbar_list_handle: UniformListScrollHandle::new(),
        }
    }
}

impl Render for ComponentGallery {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("component-gallery-scroll")
            .size_full()
            .overflow_y_scroll()
            .bg(rgb(0xf3f4f6))
            .flex()
            .flex_col()
            .child(
                div()
                    .px(px(24.0))
                    .py(px(14.0))
                    .border_b_1()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xffffff))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_size(px(20.0))
                            .text_color(rgb(0x111827))
                            .child("base_gpui component gallery"),
                    )
                    .child(
                        div().text_size(px(13.0)).text_color(rgb(0x6b7280)).child(
                            "One tab per ported component; each tab collects its usage examples.",
                        ),
                    ),
            )
            .child(
                TabsRoot::<&'static str>::new()
                    .id("gallery-nav")
                    .orientation(TabsOrientation::Vertical)
                    .default_value(Some("accordion"))
                    .flex()
                    .items_start()
                    .child(
                        TabsList::new()
                            .w(px(220.0))
                            .flex_shrink_0()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .p_3()
                            .border_r_1()
                            .border_color(rgb(0xe5e7eb))
                            .bg(rgb(0xffffff))
                            .child(nav_tab("accordion", "Accordion"))
                            .child(nav_tab("alert-dialog", "Alert Dialog"))
                            .child(nav_tab("autocomplete", "Autocomplete"))
                            .child(nav_tab("avatar", "Avatar"))
                            .child(nav_tab("button", "Button"))
                            .child(nav_tab("checkbox", "Checkbox"))
                            .child(nav_tab("checkbox-group", "Checkbox Group"))
                            .child(nav_tab("collapsible", "Collapsible"))
                            .child(nav_tab("combobox", "Combobox"))
                            .child(nav_tab("context-menu", "Context Menu"))
                            .child(nav_tab("dialog", "Dialog"))
                            .child(nav_tab("drawer", "Drawer"))
                            .child(nav_tab("field", "Field"))
                            .child(nav_tab("fieldset", "Fieldset"))
                            .child(nav_tab("form", "Form"))
                            .child(nav_tab("input", "Input"))
                            .child(nav_tab("menu", "Menu"))
                            .child(nav_tab("menubar", "Menubar"))
                            .child(nav_tab("meter", "Meter"))
                            .child(nav_tab("navigation-menu", "Navigation Menu"))
                            .child(nav_tab("number-field", "Number Field"))
                            .child(nav_tab("otp-field", "OTP Field"))
                            .child(nav_tab("popover", "Popover"))
                            .child(nav_tab("preview-card", "Preview Card"))
                            .child(nav_tab("progress", "Progress"))
                            .child(nav_tab("radio-group", "Radio Group"))
                            .child(nav_tab("scroll-area", "Scroll Area"))
                            .child(nav_tab("scrollbar", "Scrollbar"))
                            .child(nav_tab("select", "Select"))
                            .child(nav_tab("separator", "Separator"))
                            .child(nav_tab("slider", "Slider"))
                            .child(nav_tab("switch", "Switch"))
                            .child(nav_tab("tabs", "Tabs"))
                            .child(nav_tab("toast", "Toast"))
                            .child(nav_tab("toggle", "Toggle"))
                            .child(nav_tab("toggle-group", "Toggle Group"))
                            .child(nav_tab("toolbar", "Toolbar"))
                            .child(nav_tab("tooltip", "Tooltip")),
                    )
                    .child(nav_panel(
                        "accordion",
                        Vec::from([component_card(
                            "Accordion",
                            "Single-open FAQ sections with headings, triggers, and panels.",
                            accordion_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "alert-dialog",
                        Vec::from([component_card(
                            "Alert Dialog",
                            "Always-modal confirmation dialog; only Escape or an explicit action closes it.",
                            alert_dialog_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "autocomplete",
                        Vec::from([
                            component_card(
                                "Autocomplete",
                                "Text input with a filtered suggestion list (mode: List).",
                                autocomplete_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Autocomplete (inline)",
                                "Keyboard highlight inline-autocompletes the input (mode: Both).",
                                autocomplete_both_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "avatar",
                        Vec::from([component_card(
                            "Avatar",
                            "Image loading status with initials fallback and show-delay.",
                            avatar_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "button",
                        Vec::from([component_card_sized(
                            "Button",
                            "Plain pressable with disabled and focusable-when-disabled.",
                            px(460.0),
                            button_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "checkbox",
                        Vec::from([component_card(
                            "Checkbox",
                            "Unchecked and checked states with indicator rendering.",
                            checkbox_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "checkbox-group",
                        Vec::from([component_card(
                            "Checkbox Group",
                            "Shared selected values, parent state, and disabled propagation.",
                            checkbox_group_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "collapsible",
                        Vec::from([component_card(
                            "Collapsible",
                            "Disclosure trigger with controlled panel presence.",
                            collapsible_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "combobox",
                        Vec::from([
                            component_card(
                                "Combobox",
                                "Input-filtered listbox with clear, trigger, indicators, and empty state.",
                                combobox_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Multiple Combobox",
                                "Multi-select combobox with removable chips and chip keyboard navigation.",
                                multiple_combobox_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Field + Combobox",
                                "Required Field registration with a serialized Combobox value.",
                                field_combobox_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "context-menu",
                        Vec::from([component_card(
                            "Context Menu",
                            "Right-click surface opening a menu at the cursor with items, a checkbox item, and a submenu.",
                            context_menu_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "dialog",
                        Vec::from([component_card(
                            "Dialog",
                            "Modal overlay with trigger, backdrop, popup, title, description, and close.",
                            dialog_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "drawer",
                        Vec::from([
                            component_card(
                                "Drawer",
                                "Swipeable bottom panel; drag the sheet downward to dismiss it.",
                                drawer_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Drawer Snap Points",
                                "Bottom drawer that settles on 25%/50% viewport snap points on release.",
                                drawer_snap_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "field",
                        Vec::from([component_card(
                            "Field + Switch",
                            "Field label, error, description, and registered control focus.",
                            field_switch_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "fieldset",
                        Vec::from([component_card(
                            "Fieldset",
                            "Grouped fields with legend state and disabled propagation.",
                            fieldset_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "form",
                        Vec::from([component_card(
                            "Form",
                            "Form validation-mode inheritance and named Field registration.",
                            form_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "input",
                        Vec::from([
                            component_card(
                                "Input",
                                "Public Field-aware text input using Input::new().",
                                plain_input_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Field + Input",
                                "Required validation and Field-derived Input styling.",
                                input_field_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "menu",
                        Vec::from([
                            component_card(
                                "Menu",
                                "Dropdown menu with items, separator, and a group.",
                                menu_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Menu (checkbox + radio)",
                                "Checkbox items and a radio group with indicators.",
                                menu_checkbox_radio_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Menu (submenu)",
                                "Nested submenu opened by hover or ArrowRight.",
                                menu_submenu_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "menubar",
                        Vec::from([component_card(
                            "Menubar",
                            "Horizontal menu row with roving focus, hover-switch, and a submenu.",
                            menubar_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "meter",
                        Vec::from([component_card(
                            "Meter",
                            "Graphical display of a value within a known range.",
                            meter_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "navigation-menu",
                        Vec::from([component_card(
                            "Navigation Menu",
                            "Horizontal menu bar with a shared retargeting panel, hover intent, and a close-on-click link.",
                            navigation_menu_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "number-field",
                        Vec::from([
                            component_card(
                                "Number Field",
                                "Text editing, stepping, min/max, and formatted numeric value.",
                                number_field_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Field + Number Field",
                                "Required Field validation with a numeric control.",
                                field_number_field_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "otp-field",
                        Vec::from([component_card(
                            "Field + OTP Field",
                            "Six-slot one-time code with separator and required validation.",
                            field_otp_field_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "popover",
                        Vec::from([component_card(
                            "Popover",
                            "Anchored popup with trigger, title, description, arrow, and close.",
                            popover_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "preview-card",
                        Vec::from([component_card(
                            "Preview Card",
                            "Hover-opened link preview with 600ms/300ms delays, payload triggers, and a detached handle.",
                            preview_card_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "progress",
                        Vec::from([
                            component_card(
                                "Progress",
                                "Determinate task-completion display with label and value.",
                                progress_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Indeterminate Progress",
                                "Unknown-duration progress exposed as status-only state.",
                                indeterminate_progress_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "radio-group",
                        Vec::from([
                            component_card(
                                "Radio Group LTR",
                                "Direction-aware arrow key navigation in LTR mode.",
                                radio_group_demo("ltr", TextDirection::Ltr, "standard"),
                            )
                            .into_any_element(),
                            component_card(
                                "Radio Group RTL",
                                "Direction-aware arrow key navigation in RTL mode.",
                                radio_group_demo("rtl", TextDirection::Rtl, "express"),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "scroll-area",
                        Vec::from([component_card(
                            "Scroll Area",
                            "Compound scroll container with hover/scroll-styled scrollbars.",
                            scroll_area_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "scrollbar",
                        Vec::from([
                            component_card(
                                "Scrollbar (vertical)",
                                "Overlay scrollbar primitive over a tracked ScrollHandle.",
                                scrollbar_vertical_demo(&self.scrollbar_vertical_handle),
                            )
                            .into_any_element(),
                            component_card(
                                "Scrollbar (both axes)",
                                "Both tracks plus the corner over an overflow_scroll container.",
                                scrollbar_both_axes_demo(&self.scrollbar_both_handle),
                            )
                            .into_any_element(),
                            component_card(
                                "Scrollbar (uniform list)",
                                "Scrollbar over a virtualized uniform_list scroll handle.",
                                scrollbar_uniform_list_demo(&self.scrollbar_list_handle),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "select",
                        Vec::from([
                            component_card(
                                "Select",
                                "Trigger, value, popup/list, item labels, and indicator state.",
                                select_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Multiple Select",
                                "Ordered multi-value toggling with item indicators.",
                                multiple_select_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Field + Select",
                                "Required Field registration with a serialized Select value.",
                                field_select_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "separator",
                        Vec::from([component_card(
                            "Separator",
                            "Shared horizontal and vertical visual dividers.",
                            separator_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "slider",
                        Vec::from([
                            component_card(
                                "Slider",
                                "Single-value pointer and keyboard driven slider.",
                                slider_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Range Slider",
                                "Two-thumb range slider with push collision behavior.",
                                range_slider_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Field + Slider",
                                "Field label, registered slider control, and error slot.",
                                field_slider_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "switch",
                        Vec::from([component_card(
                            "Switch",
                            "Toggle state, focus styling, thumb state, and callbacks.",
                            switch_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "tabs",
                        Vec::from([component_card(
                            "Tabs",
                            "Keyboard-selectable tab list with indicator and panels.",
                            tabs_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "toast",
                        Vec::from([component_card(
                            "Toast",
                            "Queued notifications: add/upsert/promise/close-all via the imperative manager; hover the stack to pause dismissal.",
                            toast_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "toggle",
                        Vec::from([component_card(
                            "Toggle",
                            "Two-state pressable button with pressed styling.",
                            toggle_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "toggle-group",
                        Vec::from([
                            component_card(
                                "Toggle Group (multiple)",
                                "Grouped toggles where any number of items can be pressed.",
                                toggle_group_demo(),
                            )
                            .into_any_element(),
                            component_card(
                                "Toggle Group (single)",
                                "Pressing one item releases the previously pressed item.",
                                single_toggle_group_demo(),
                            )
                            .into_any_element(),
                        ]),
                    ))
                    .child(nav_panel(
                        "toolbar",
                        Vec::from([component_card_sized(
                            "Toolbar",
                            "Roving-focus toolbar with buttons, a group, a separator, a link, and an input.",
                            px(560.0),
                            toolbar_demo(),
                        )
                        .into_any_element()]),
                    ))
                    .child(nav_panel(
                        "tooltip",
                        Vec::from([component_card(
                            "Tooltip",
                            "Hover/focus visual hint with provider delay and trigger-bounds positioning.",
                            tooltip_demo(),
                        )
                        .into_any_element()]),
                    )),
            )
    }
}

fn nav_tab(value: &'static str, label: &'static str) -> TabsTab<&'static str> {
    TabsTab::new()
        .id(value)
        .value(value)
        .px_3()
        .py_2()
        .rounded_md()
        .text_size(px(13.0))
        .text_color(rgb(0x374151))
        .style_with_state(|state, tab| {
            if state.active {
                tab.bg(rgb(0xe5e7eb)).text_color(rgb(0x111827))
            } else if state.highlighted {
                tab.bg(rgb(0xf3f4f6))
            } else {
                tab
            }
        })
        .child(label)
}

fn nav_panel(value: &'static str, cards: Vec<AnyElement>) -> TabsPanel<&'static str> {
    TabsPanel::new().value(value).flex_1().min_w(px(0.0)).child(
        div()
            .w_full()
            .p(px(24.0))
            .flex()
            .flex_wrap()
            .items_start()
            .gap_3()
            .children(cards),
    )
}

fn slider_body(thumb_count: usize) -> SliderControl {
    let mut control = SliderControl::new().w_full().h(px(20.0)).child(
        SliderTrack::new()
            .absolute()
            .top(px(8.0))
            .left(px(0.0))
            .right(px(0.0))
            .h(px(4.0))
            .rounded_full()
            .bg(rgb(0xd1d5db))
            .child(
                SliderIndicator::new()
                    .h(px(4.0))
                    .rounded_full()
                    .bg(rgb(0x2563eb)),
            ),
    );
    for _ in 0..thumb_count {
        control = control.child(
            SliderThumb::new()
                .top(px(4.0))
                .w(px(12.0))
                .h(px(12.0))
                .rounded_full()
                .bg(rgb(0xffffff))
                .border_2()
                .border_color(rgb(0x2563eb))
                .style_with_state(|state, thumb| {
                    if state.focused {
                        thumb.border_color(rgb(0x1d4ed8)).shadow_md()
                    } else {
                        thumb
                    }
                }),
        );
    }
    control
}

fn slider_demo() -> impl IntoElement {
    SliderRoot::new()
        .id("example-slider")
        .default_value(SliderValues::Single(40.0))
        .flex()
        .flex_col()
        .gap_2()
        .child(
            SliderLabel::new()
                .text_size(px(13.0))
                .text_color(rgb(0x374151))
                .child("Volume"),
        )
        .child(
            SliderValue::new()
                .text_size(px(12.0))
                .text_color(rgb(0x6b7280)),
        )
        .child(slider_body(1))
}

fn range_slider_demo() -> impl IntoElement {
    SliderRoot::new()
        .id("example-range-slider")
        .default_value(SliderValues::Range(Vec::from([20.0, 80.0])))
        .flex()
        .flex_col()
        .gap_2()
        .child(
            SliderValue::new()
                .text_size(px(12.0))
                .text_color(rgb(0x6b7280)),
        )
        .child(slider_body(2))
}

fn progress_demo() -> impl IntoElement {
    ProgressRoot::new()
        .id("example-progress")
        .value(Some(65.0))
        .flex()
        .flex_col()
        .gap_2()
        .child(
            ProgressLabel::new()
                .text_size(px(13.0))
                .text_color(rgb(0x374151))
                .child("Export data"),
        )
        .child(
            ProgressValue::new()
                .text_size(px(12.0))
                .text_color(rgb(0x6b7280)),
        )
        .child(
            ProgressTrack::new()
                .w_full()
                .h(px(4.0))
                .rounded_full()
                .bg(rgb(0xd1d5db))
                .child(
                    ProgressIndicator::new()
                        .h(px(4.0))
                        .rounded_full()
                        .bg(rgb(0x2563eb)),
                ),
        )
}

fn meter_demo() -> impl IntoElement {
    MeterRoot::new()
        .id("example-meter")
        .value(24.0)
        .flex()
        .flex_col()
        .gap_2()
        .child(
            MeterLabel::new()
                .text_size(px(13.0))
                .text_color(rgb(0x374151))
                .child("Battery level"),
        )
        .child(
            MeterValue::new()
                .text_size(px(12.0))
                .text_color(rgb(0x6b7280)),
        )
        .child(
            MeterTrack::new()
                .w_full()
                .h(px(4.0))
                .rounded_full()
                .bg(rgb(0xd1d5db))
                .child(
                    MeterIndicator::new()
                        .h(px(4.0))
                        .rounded_full()
                        .bg(rgb(0x2563eb)),
                ),
        )
}

fn indeterminate_progress_demo() -> impl IntoElement {
    ProgressRoot::new()
        .id("example-indeterminate-progress")
        .flex()
        .flex_col()
        .gap_2()
        .child(
            ProgressLabel::new()
                .text_size(px(13.0))
                .text_color(rgb(0x374151))
                .child("Preparing download\u{2026}"),
        )
        .child(
            ProgressTrack::new()
                .w_full()
                .h(px(4.0))
                .rounded_full()
                .bg(rgb(0xd1d5db))
                .child(
                    ProgressIndicator::new()
                        .h(px(4.0))
                        .rounded_full()
                        .style_with_state(|_state, indicator| {
                            // Status-only indeterminate treatment: style it here.
                            indicator.w(gpui::relative(0.25)).bg(rgb(0x93c5fd))
                        }),
                ),
        )
}

fn field_slider_demo() -> impl IntoElement {
    FieldRoot::new()
        .id("example-slider-field")
        .flex()
        .flex_col()
        .gap_2()
        .child(
            FieldLabel::new()
                .text_size(px(13.0))
                .text_color(rgb(0x374151))
                .child("Brightness"),
        )
        .child(
            SliderRoot::new()
                .id("example-field-slider")
                .name("brightness")
                .default_value(SliderValues::Single(60.0))
                .child(slider_body(1)),
        )
        .child(
            FieldError::new()
                .text_color(rgb(0xdc2626))
                .text_size(px(12.0)),
        )
}

fn avatar_demo() -> impl IntoElement {
    let avatar_shell = |id: &'static str| {
        AvatarRoot::new()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .w(px(40.0))
            .h(px(40.0))
            .rounded_full()
            .overflow_hidden()
            .bg(rgb(0xe5e7eb))
            .text_size(px(13.0))
            .text_color(rgb(0x111827))
    };

    div()
        .flex()
        .gap_3()
        .items_center()
        .child(
            avatar_shell("avatar-demo-image")
                .child(
                    AvatarImage::new(
                        "https://images.unsplash.com/photo-1543610892-0b1f7e6d8ac1?w=128&h=128",
                    )
                    .w(px(40.0))
                    .h(px(40.0)),
                )
                .child(AvatarFallback::new().child("LT")),
        )
        .child(
            avatar_shell("avatar-demo-broken")
                .child(AvatarImage::new("").w(px(40.0)).h(px(40.0)))
                .child(AvatarFallback::new().child("LT")),
        )
        .child(
            avatar_shell("avatar-demo-delay")
                .child(AvatarImage::new("").w(px(40.0)).h(px(40.0)))
                .child(
                    AvatarFallback::new()
                        .delay(Duration::from_millis(600))
                        .child("LT"),
                ),
        )
}

fn component_card(
    title: &'static str,
    description: &'static str,
    content: impl IntoElement,
) -> impl IntoElement {
    component_card_sized(title, description, px(300.0), content)
}

fn component_card_sized(
    title: &'static str,
    description: &'static str,
    width: Pixels,
    content: impl IntoElement,
) -> impl IntoElement {
    div()
        .w(width)
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

fn menu_item_style(highlighted: bool, item: gpui::Div) -> gpui::Div {
    let item = item
        .px_3()
        .py_1p5()
        .rounded_sm()
        .text_size(px(13.0))
        .text_color(rgb(0x111827));
    if highlighted {
        item.bg(rgb(0xe5e7eb))
    } else {
        item
    }
}

fn menu_trigger_button(label: &'static str) -> MenuTrigger<()> {
    MenuTrigger::<()>::new()
        .id(label)
        .px_3()
        .py_2()
        .rounded_md()
        .border_1()
        .border_color(rgb(0x9ca3af))
        .bg(rgb(0xffffff))
        .text_size(px(13.0))
        .text_color(rgb(0x111827))
        .child(label)
}

fn menu_popup_frame() -> MenuPopup<()> {
    MenuPopup::<()>::new()
        .w(px(200.0))
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .bg(rgb(0xffffff))
        .shadow_lg()
        .p_1()
        .flex()
        .flex_col()
}

fn menu_demo() -> impl IntoElement {
    MenuRoot::<()>::new()
        .id("gallery-menu")
        .modal(false)
        .child(menu_trigger_button("Open menu"))
        .child(
            MenuPortal::<()>::new().child(
                MenuPositioner::<()>::new().side_offset(px(6.0)).child(
                    menu_popup_frame()
                        .child(
                            MenuItem::<()>::new()
                                .id("new-file")
                                .label("New file")
                                .style_with_state(|state, item| {
                                    menu_item_style(state.highlighted, item)
                                })
                                .child("New file"),
                        )
                        .child(
                            MenuItem::<()>::new()
                                .id("open-file")
                                .label("Open file")
                                .style_with_state(|state, item| {
                                    menu_item_style(state.highlighted, item)
                                })
                                .child("Open file"),
                        )
                        .child(MenuSeparator::new().h(px(1.0)).my_1().bg(rgb(0xe5e7eb)))
                        .child(
                            MenuGroup::<()>::new()
                                .child(
                                    MenuGroupLabel::<()>::new()
                                        .label("Danger")
                                        .px_3()
                                        .py_1()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0x6b7280))
                                        .child("Danger"),
                                )
                                .child(
                                    MenuItem::<()>::new()
                                        .id("delete")
                                        .label("Delete")
                                        .style_with_state(|state, item| {
                                            menu_item_style(state.highlighted, item)
                                        })
                                        .child("Delete"),
                                ),
                        ),
                ),
            ),
        )
}

fn menu_checkbox_radio_demo() -> impl IntoElement {
    MenuRoot::<()>::new()
        .id("gallery-menu-check-radio")
        .modal(false)
        .child(menu_trigger_button("View options"))
        .child(
            MenuPortal::<()>::new().child(
                MenuPositioner::<()>::new().side_offset(px(6.0)).child(
                    menu_popup_frame()
                        .child(
                            MenuCheckboxItem::<()>::new()
                                .id("show-toolbar")
                                .label("Show toolbar")
                                .default_checked(true)
                                .style_with_state(|state, item| {
                                    menu_item_style(state.highlighted, item)
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                })
                                .child(
                                    MenuCheckboxItemIndicator::<()>::new()
                                        .w(px(8.0))
                                        .h(px(8.0))
                                        .rounded_full()
                                        .bg(rgb(0x2563eb)),
                                )
                                .child_any("Show toolbar"),
                        )
                        .child(MenuSeparator::new().h(px(1.0)).my_1().bg(rgb(0xe5e7eb)))
                        .child(
                            MenuRadioGroup::<(), &'static str>::new()
                                .default_value(Some("list"))
                                .child(
                                    MenuRadioItem::<(), &'static str>::new()
                                        .id("view-list")
                                        .value("list")
                                        .label("List view")
                                        .style_with_state(|state, item| {
                                            menu_item_style(state.highlighted, item)
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                        })
                                        .child(
                                            MenuRadioItemIndicator::<(), &'static str>::new()
                                                .w(px(8.0))
                                                .h(px(8.0))
                                                .rounded_full()
                                                .bg(rgb(0x16a34a)),
                                        )
                                        .child_any("List view"),
                                )
                                .child(
                                    MenuRadioItem::<(), &'static str>::new()
                                        .id("view-grid")
                                        .value("grid")
                                        .label("Grid view")
                                        .style_with_state(|state, item| {
                                            menu_item_style(state.highlighted, item)
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                        })
                                        .child(
                                            MenuRadioItemIndicator::<(), &'static str>::new()
                                                .w(px(8.0))
                                                .h(px(8.0))
                                                .rounded_full()
                                                .bg(rgb(0x16a34a)),
                                        )
                                        .child_any("Grid view"),
                                ),
                        ),
                ),
            ),
        )
}

fn menu_submenu_demo() -> impl IntoElement {
    MenuRoot::<()>::new()
        .id("gallery-menu-submenu")
        .modal(false)
        .child(menu_trigger_button("Share"))
        .child(
            MenuPortal::<()>::new().child(
                MenuPositioner::<()>::new().side_offset(px(6.0)).child(
                    menu_popup_frame()
                        .child(
                            MenuItem::<()>::new()
                                .id("copy-link")
                                .label("Copy link")
                                .style_with_state(|state, item| {
                                    menu_item_style(state.highlighted, item)
                                })
                                .child("Copy link"),
                        )
                        .child(
                            MenuSubmenuRoot::<()>::new()
                                .id("share-submenu")
                                .child(
                                    MenuSubmenuTrigger::<()>::new()
                                        .id("share-with")
                                        .label("Share with")
                                        .style_with_state(|state, item| {
                                            menu_item_style(state.highlighted || state.open, item)
                                        })
                                        .child("Share with \u{2192}"),
                                )
                                .child(
                                    MenuPortal::<()>::new().child(
                                        MenuPositioner::<()>::new().side_offset(px(2.0)).child(
                                            menu_popup_frame()
                                                .child(
                                                    MenuItem::<()>::new()
                                                        .id("share-email")
                                                        .label("Email")
                                                        .style_with_state(|state, item| {
                                                            menu_item_style(state.highlighted, item)
                                                        })
                                                        .child("Email"),
                                                )
                                                .child(
                                                    MenuItem::<()>::new()
                                                        .id("share-message")
                                                        .label("Message")
                                                        .style_with_state(|state, item| {
                                                            menu_item_style(state.highlighted, item)
                                                        })
                                                        .child("Message"),
                                                ),
                                        ),
                                    ),
                                ),
                        ),
                ),
            ),
        )
}

fn context_menu_demo() -> impl IntoElement {
    ContextMenuRoot::<()>::new()
        .id("gallery-context-menu")
        .child(
            ContextMenuTrigger::<()>::new()
                .id("gallery-context-menu-trigger")
                .px_8()
                .py_6()
                .rounded_md()
                .border_1()
                .border_color(rgb(0x9ca3af))
                .bg(rgb(0xf9fafb))
                .text_size(px(13.0))
                .text_color(rgb(0x6b7280))
                .style_with_state(|state, area| match state.open {
                    true => area.border_color(rgb(0x2563eb)),
                    false => area,
                })
                .child_any("Right-click here"),
        )
        .child(
            ContextMenuPortal::<()>::new().child(
                ContextMenuPositioner::<()>::new().child(
                    menu_popup_frame()
                        .child(
                            ContextMenuItem::<()>::new()
                                .id("ctx-cut")
                                .label("Cut")
                                .style_with_state(|state, item| {
                                    menu_item_style(state.highlighted, item)
                                })
                                .child("Cut"),
                        )
                        .child(
                            ContextMenuItem::<()>::new()
                                .id("ctx-copy")
                                .label("Copy")
                                .style_with_state(|state, item| {
                                    menu_item_style(state.highlighted, item)
                                })
                                .child("Copy"),
                        )
                        .child(
                            ContextMenuSeparator::new()
                                .h(px(1.0))
                                .my_1()
                                .bg(rgb(0xe5e7eb)),
                        )
                        .child(
                            ContextMenuCheckboxItem::<()>::new()
                                .id("ctx-word-wrap")
                                .label("Word wrap")
                                .default_checked(true)
                                .style_with_state(|state, item| {
                                    menu_item_style(state.highlighted, item)
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                })
                                .child(
                                    ContextMenuCheckboxItemIndicator::<()>::new()
                                        .w(px(8.0))
                                        .h(px(8.0))
                                        .rounded_full()
                                        .bg(rgb(0x2563eb)),
                                )
                                .child_any("Word wrap"),
                        )
                        .child(
                            ContextMenuSubmenuRoot::<()>::new()
                                .id("ctx-share-submenu")
                                .child(
                                    ContextMenuSubmenuTrigger::<()>::new()
                                        .id("ctx-share-with")
                                        .label("Share with")
                                        .style_with_state(|state, item| {
                                            menu_item_style(state.highlighted || state.open, item)
                                        })
                                        .child("Share with \u{2192}"),
                                )
                                .child(
                                    ContextMenuPortal::<()>::new().child(
                                        ContextMenuPositioner::<()>::new()
                                            .side_offset(px(2.0))
                                            .child(
                                                menu_popup_frame()
                                                    .child(
                                                        ContextMenuItem::<()>::new()
                                                            .id("ctx-share-email")
                                                            .label("Email")
                                                            .style_with_state(|state, item| {
                                                                menu_item_style(
                                                                    state.highlighted,
                                                                    item,
                                                                )
                                                            })
                                                            .child("Email"),
                                                    )
                                                    .child(
                                                        ContextMenuItem::<()>::new()
                                                            .id("ctx-share-message")
                                                            .label("Message")
                                                            .style_with_state(|state, item| {
                                                                menu_item_style(
                                                                    state.highlighted,
                                                                    item,
                                                                )
                                                            })
                                                            .child("Message"),
                                                    ),
                                            ),
                                    ),
                                ),
                        ),
                ),
            ),
        )
}

fn menubar_menu(
    id: &'static str,
    label: &'static str,
    items: Vec<(&'static str, &'static str)>,
) -> MenuRoot<()> {
    let mut popup = menu_popup_frame();
    for (item_id, item_label) in items {
        popup = popup.child(
            MenuItem::<()>::new()
                .id(item_id)
                .label(item_label)
                .style_with_state(|state, item| menu_item_style(state.highlighted, item))
                .child(item_label),
        );
    }
    MenuRoot::<()>::new()
        .id(id)
        .child(menu_trigger_button(label))
        .child(
            MenuPortal::<()>::new().child(
                MenuPositioner::<()>::new()
                    .side_offset(px(4.0))
                    .child(popup),
            ),
        )
}

fn menubar_demo() -> impl IntoElement {
    Menubar::new()
        .id("gallery-menubar")
        .modal(false)
        .flex()
        .flex_row()
        .gap_1()
        .p_1()
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .bg(rgb(0xf9fafb))
        .child(menubar_menu(
            "menubar-file",
            "File",
            vec![("mb-new", "New"), ("mb-open", "Open"), ("mb-save", "Save")],
        ))
        .child(
            MenuRoot::<()>::new()
                .id("menubar-edit")
                .child(menu_trigger_button("Edit"))
                .child(
                    MenuPortal::<()>::new().child(
                        MenuPositioner::<()>::new().side_offset(px(4.0)).child(
                            menu_popup_frame()
                                .child(
                                    MenuItem::<()>::new()
                                        .id("mb-undo")
                                        .label("Undo")
                                        .style_with_state(|state, item| {
                                            menu_item_style(state.highlighted, item)
                                        })
                                        .child("Undo"),
                                )
                                .child(
                                    MenuSubmenuRoot::<()>::new()
                                        .id("menubar-find")
                                        .child(
                                            MenuSubmenuTrigger::<()>::new()
                                                .id("mb-find")
                                                .label("Find")
                                                .style_with_state(|state, item| {
                                                    menu_item_style(
                                                        state.highlighted || state.open,
                                                        item,
                                                    )
                                                })
                                                .child("Find \u{2192}"),
                                        )
                                        .child(
                                            MenuPortal::<()>::new().child(
                                                MenuPositioner::<()>::new()
                                                    .side_offset(px(2.0))
                                                    .child(
                                                        menu_popup_frame().child(
                                                            MenuItem::<()>::new()
                                                                .id("mb-find-next")
                                                                .label("Find next")
                                                                .style_with_state(|state, item| {
                                                                    menu_item_style(
                                                                        state.highlighted,
                                                                        item,
                                                                    )
                                                                })
                                                                .child("Find next"),
                                                        ),
                                                    ),
                                            ),
                                        ),
                                ),
                        ),
                    ),
                ),
        )
        .child(menubar_menu(
            "menubar-view",
            "View",
            vec![("mb-zoom-in", "Zoom in"), ("mb-zoom-out", "Zoom out")],
        ))
        .child(menubar_menu("menubar-help", "Help", vec![("mb-about", "About")]).disabled(true))
}

fn popover_demo() -> impl IntoElement {
    PopoverRoot::<()>::new()
        .id("gallery-popover")
        .flex()
        .items_center()
        .gap_2()
        .child(
            PopoverTrigger::<()>::new()
                .id("details")
                .px_3()
                .py_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(0x9ca3af))
                .bg(rgb(0xffffff))
                .text_size(px(13.0))
                .text_color(rgb(0x111827))
                .child("Open details"),
        )
        .child(
            PopoverTrigger::<()>::new()
                .id("help")
                .open_on_hover(true)
                .delay(Duration::from_millis(150))
                .close_delay(Duration::from_millis(150))
                .px_3()
                .py_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(0x9ca3af))
                .bg(rgb(0xffffff))
                .text_size(px(13.0))
                .text_color(rgb(0x111827))
                .child("Open help"),
        )
        .child(
            PopoverPortal::<()>::new().child(
                PopoverPositioner::<()>::new().side_offset(px(8.0)).child(
                    PopoverPopup::<()>::new()
                        .w(px(220.0))
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(0xd1d5db))
                        .bg(rgb(0xffffff))
                        .shadow_lg()
                        .p_3()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            PopoverArrow::<()>::new()
                                .w(px(10.0))
                                .h(px(10.0))
                                .bg(rgb(0xffffff)),
                        )
                        .child(
                            PopoverTitle::<()>::new()
                                .text_size(px(14.0))
                                .text_color(rgb(0x111827))
                                .child("Popover title"),
                        )
                        .child(
                            PopoverDescription::<()>::new()
                                .text_size(px(12.0))
                                .text_color(rgb(0x6b7280))
                                .child("This popup is ported with GPUI-native anchored rendering."),
                        )
                        .child(
                            PopoverClose::<()>::new()
                                .self_start()
                                .mt_1()
                                .px_2()
                                .py_1()
                                .rounded_sm()
                                .bg(rgb(0xe5e7eb))
                                .text_size(px(12.0))
                                .text_color(rgb(0x111827))
                                .child("Close"),
                        ),
                ),
            ),
        )
}

fn dialog_demo() -> impl IntoElement {
    DialogRoot::<()>::new()
        .id("gallery-dialog")
        .flex()
        .items_center()
        .gap_2()
        .child(
            DialogTrigger::<()>::new()
                .id("open")
                .px_3()
                .py_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(0x9ca3af))
                .bg(rgb(0xffffff))
                .text_size(px(13.0))
                .text_color(rgb(0x111827))
                .child("Open dialog"),
        )
        .child(
            DialogPortal::<()>::new()
                .child(
                    DialogBackdrop::<()>::new().style_with_state(|_, backdrop| {
                        backdrop
                            .absolute()
                            .top_0()
                            .left_0()
                            .w_full()
                            .h_full()
                            .bg(rgb(0x111827))
                            .opacity(0.35)
                    }),
                )
                .child(
                    DialogViewport::<()>::new()
                        .absolute()
                        .top_0()
                        .left_0()
                        .w_full()
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            DialogPopup::<()>::new()
                                .w(px(260.0))
                                .rounded_lg()
                                .border_1()
                                .border_color(rgb(0xd1d5db))
                                .bg(rgb(0xffffff))
                                .shadow_lg()
                                .p_4()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    DialogTitle::<()>::new()
                                        .text_size(px(15.0))
                                        .text_color(rgb(0x111827))
                                        .child("Dialog title"),
                                )
                                .child(
                                    DialogDescription::<()>::new()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x6b7280))
                                        .child("This modal is drawn through GPUI deferred overlay rendering."),
                                )
                                .child(
                                    DialogClose::<()>::new()
                                        .self_start()
                                        .mt_2()
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .bg(rgb(0xe5e7eb))
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x111827))
                                        .child("Close"),
                                ),
                        ),
                ),
        )
}

fn drawer_demo() -> impl IntoElement {
    DrawerRoot::<()>::new()
        .id("gallery-drawer")
        .flex()
        .items_center()
        .gap_2()
        .child(
            DrawerTrigger::<()>::new()
                .id("open")
                .px_3()
                .py_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(0x9ca3af))
                .bg(rgb(0xffffff))
                .text_size(px(13.0))
                .text_color(rgb(0x111827))
                .child("Open drawer"),
        )
        .child(
            DrawerPortal::<()>::new()
                .child(DrawerBackdrop::<()>::new().style_with_state(|state, backdrop| {
                    backdrop
                        .absolute()
                        .top_0()
                        .left_0()
                        .w_full()
                        .h_full()
                        .bg(rgb(0x111827))
                        .opacity(0.35 * (1.0 - state.swipe_progress))
                }))
                .child(
                    DrawerViewport::<()>::new()
                        .id("gallery-drawer-viewport")
                        .absolute()
                        .top_0()
                        .left_0()
                        .w_full()
                        .h_full()
                        .flex()
                        .flex_col()
                        .justify_end()
                        .child(
                            DrawerPopup::<()>::new()
                                .style_with_state(|state, popup| {
                                    popup.mt(state.swipe_movement.y.max(px(0.0)))
                                })
                                .w_full()
                                .rounded_t_lg()
                                .border_1()
                                .border_color(rgb(0xd1d5db))
                                .bg(rgb(0xffffff))
                                .shadow_lg()
                                .p_4()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    DrawerTitle::<()>::new()
                                        .text_size(px(15.0))
                                        .text_color(rgb(0x111827))
                                        .child("Drawer title"),
                                )
                                .child(
                                    DrawerContent::<()>::new()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x6b7280))
                                        .child("Drag anywhere outside this content to swipe the drawer down and dismiss it."),
                                )
                                .child(
                                    DrawerClose::<()>::new()
                                        .self_start()
                                        .mt_2()
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .bg(rgb(0xe5e7eb))
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x111827))
                                        .child("Close"),
                                ),
                        ),
                ),
        )
}

fn drawer_snap_demo() -> impl IntoElement {
    DrawerRoot::<()>::new()
        .id("gallery-drawer-snap")
        .snap_points(vec![
            DrawerSnapPoint::Fraction(0.25),
            DrawerSnapPoint::Fraction(0.5),
        ])
        .flex()
        .items_center()
        .gap_2()
        .child(
            DrawerTrigger::<()>::new()
                .id("open")
                .px_3()
                .py_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(0x9ca3af))
                .bg(rgb(0xffffff))
                .text_size(px(13.0))
                .text_color(rgb(0x111827))
                .child("Open snap drawer"),
        )
        .child(
            DrawerPortal::<()>::new()
                .child(DrawerBackdrop::<()>::new().style_with_state(|state, backdrop| {
                    backdrop
                        .absolute()
                        .top_0()
                        .left_0()
                        .w_full()
                        .h_full()
                        .bg(rgb(0x111827))
                        .opacity(0.35 * (1.0 - state.swipe_progress))
                }))
                .child(
                    DrawerViewport::<()>::new()
                        .id("gallery-drawer-snap-viewport")
                        .absolute()
                        .top_0()
                        .left_0()
                        .w_full()
                        .h_full()
                        .flex()
                        .flex_col()
                        .justify_end()
                        .child(
                            DrawerPopup::<()>::new()
                                .style_with_state(|state, popup| {
                                    let offset = state.snap_point_offset
                                        + state.swipe_movement.y.max(px(0.0));
                                    popup.mt(offset.max(px(0.0)))
                                })
                                .w_full()
                                .h_full()
                                .rounded_t_lg()
                                .border_1()
                                .border_color(rgb(0xd1d5db))
                                .bg(rgb(0xffffff))
                                .shadow_lg()
                                .p_4()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    DrawerTitle::<()>::new()
                                        .text_size(px(15.0))
                                        .text_color(rgb(0x111827))
                                        .child("Snap-point drawer"),
                                )
                                .child(
                                    DrawerContent::<()>::new()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x6b7280))
                                        .child("Release the drag near a snap point to settle there; drag far down to close."),
                                ),
                        ),
                ),
        )
}

fn alert_dialog_demo() -> impl IntoElement {
    AlertDialogRoot::<()>::new()
        .id("gallery-alert-dialog")
        .flex()
        .items_center()
        .gap_2()
        .child(
            AlertDialogTrigger::<()>::new()
                .id("open")
                .px_3()
                .py_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(0x9ca3af))
                .bg(rgb(0xffffff))
                .text_size(px(13.0))
                .text_color(rgb(0x111827))
                .child("Discard draft…"),
        )
        .child(
            AlertDialogPortal::<()>::new()
                .child(
                    AlertDialogBackdrop::<()>::new().style_with_state(|_, backdrop| {
                        backdrop
                            .absolute()
                            .top_0()
                            .left_0()
                            .w_full()
                            .h_full()
                            .bg(rgb(0x111827))
                            .opacity(0.35)
                    }),
                )
                .child(
                    AlertDialogViewport::<()>::new()
                        .absolute()
                        .top_0()
                        .left_0()
                        .w_full()
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            AlertDialogPopup::<()>::new()
                                .w(px(280.0))
                                .rounded_lg()
                                .border_1()
                                .border_color(rgb(0xd1d5db))
                                .bg(rgb(0xffffff))
                                .shadow_lg()
                                .p_4()
                                .flex()
                                .flex_wrap()
                                .justify_end()
                                .gap_2()
                                .child(
                                    AlertDialogTitle::<()>::new()
                                        .w_full()
                                        .text_size(px(15.0))
                                        .text_color(rgb(0x111827))
                                        .child("Discard draft?"),
                                )
                                .child(
                                    AlertDialogDescription::<()>::new()
                                        .w_full()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x6b7280))
                                        .child("This cannot be undone. Clicking outside will not dismiss this dialog."),
                                )
                                .child(
                                    AlertDialogClose::<()>::new()
                                        .id("alert-cancel")
                                        .mt_2()
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .bg(rgb(0xe5e7eb))
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x111827))
                                        .child("Cancel"),
                                )
                                .child(
                                    AlertDialogClose::<()>::new()
                                        .id("alert-discard")
                                        .mt_2()
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .bg(rgb(0xdc2626))
                                        .text_size(px(12.0))
                                        .text_color(rgb(0xffffff))
                                        .child("Discard"),
                                ),
                        ),
                ),
        )
}

fn preview_card_demo() -> impl IntoElement {
    let handle = create_preview_card_handle::<&'static str>();
    let handle_for_trigger = handle.clone();

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            // Basic: single trigger, starts closed, opens on hover/focus.
            PreviewCardRoot::<&'static str>::new()
                .id("gallery-preview-card-basic")
                .flex()
                .gap_2()
                .child(
                    preview_card_gallery_trigger("gallery-preview-card-basic-trigger", "@base_ui")
                        .payload(
                            "Base UI: unstyled UI components for building accessible web apps.",
                        ),
                )
                .child(preview_card_gallery_popup()),
        )
        .child(
            // Payload/multi-trigger: two triggers share one card and swap payloads.
            PreviewCardRoot::<&'static str>::new()
                .id("gallery-preview-card-multi")
                .flex()
                .gap_2()
                .child(
                    preview_card_gallery_trigger("gallery-preview-card-multi-one", "Alice")
                        .payload("Alice — maintains the design system."),
                )
                .child(
                    preview_card_gallery_trigger("gallery-preview-card-multi-two", "Bob")
                        .payload("Bob — works on the rendering engine."),
                )
                .child(preview_card_gallery_popup()),
        )
        .child(
            // Detached handle: the trigger lives outside the root.
            div()
                .flex()
                .gap_2()
                .child(
                    PreviewCardTrigger::<&'static str>::new()
                        .id("gallery-preview-card-detached-trigger")
                        .handle(handle_for_trigger)
                        .payload("Detached trigger connected through a handle.")
                        .px_3()
                        .py_2()
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(0x9ca3af))
                        .bg(rgb(0xffffff))
                        .text_size(px(13.0))
                        .text_color(rgb(0x111827))
                        .child("Detached"),
                )
                .child(
                    PreviewCardRoot::<&'static str>::new()
                        .id("gallery-preview-card-detached")
                        .handle(handle)
                        .child(preview_card_gallery_popup()),
                ),
        )
}

fn preview_card_gallery_trigger(
    id: &'static str,
    label: &'static str,
) -> PreviewCardTrigger<&'static str> {
    PreviewCardTrigger::<&'static str>::new()
        .id(id)
        .px_3()
        .py_2()
        .rounded_md()
        .border_1()
        .border_color(rgb(0x9ca3af))
        .bg(rgb(0xffffff))
        .text_size(px(13.0))
        .text_color(rgb(0x1d4ed8))
        .child(label)
}

fn preview_card_gallery_popup() -> PreviewCardPortal<&'static str> {
    PreviewCardPortal::<&'static str>::new().child(
        PreviewCardPositioner::<&'static str>::new().child(
            PreviewCardPopup::<&'static str>::new()
                .rounded_md()
                .bg(rgb(0xffffff))
                .border_1()
                .border_color(rgb(0xd1d5db))
                .text_color(rgb(0x111827))
                .text_size(px(12.0))
                .py_2()
                .px_3()
                .w(px(240.0))
                .shadow_lg()
                .child(PreviewCardViewport::<&'static str>::new().payload_content(
                    |payload, _window, _cx| {
                        div()
                            .child(payload.copied().unwrap_or("Hover a link to preview it."))
                            .into_any_element()
                    },
                )),
        ),
    )
}

fn navigation_menu_demo() -> impl IntoElement {
    NavigationMenuRoot::<&'static str>::new()
        .id("gallery-navigation-menu")
        .child(
            NavigationMenuList::<&'static str>::new()
                .flex()
                .gap_2()
                .child(navigation_menu_gallery_item(
                    "overview",
                    "Overview",
                    "Product overview: a short panel.",
                    160.0,
                ))
                .child(navigation_menu_gallery_item(
                    "docs",
                    "Docs",
                    "Documentation: a wider panel with more room for links and summaries.",
                    260.0,
                ))
                .child(
                    NavigationMenuLink::<&'static str>::new()
                        .active(true)
                        .close_on_click(true)
                        .px_3()
                        .py_2()
                        .rounded_md()
                        .text_size(px(13.0))
                        .text_color(rgb(0x1d4ed8))
                        .child("Blog"),
                ),
        )
        .child(
            NavigationMenuPortal::<&'static str>::new().child(
                NavigationMenuPositioner::<&'static str>::new().child(
                    NavigationMenuPopup::<&'static str>::new()
                        .rounded_md()
                        .bg(rgb(0xffffff))
                        .border_1()
                        .border_color(rgb(0xd1d5db))
                        .text_color(rgb(0x111827))
                        .text_size(px(12.0))
                        .py_2()
                        .px_3()
                        .shadow_lg()
                        .child(NavigationMenuArrow::<&'static str>::new().bg(rgb(0xd1d5db)))
                        .child(NavigationMenuViewport::<&'static str>::new()),
                ),
            ),
        )
}

fn navigation_menu_gallery_item(
    value: &'static str,
    label: &'static str,
    content: &'static str,
    content_width: f32,
) -> NavigationMenuItem<&'static str> {
    NavigationMenuItem::<&'static str>::new()
        .value(value)
        .child(
            NavigationMenuTrigger::<&'static str>::new()
                .flex()
                .items_center()
                .gap_1()
                .px_3()
                .py_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(0x9ca3af))
                .bg(rgb(0xffffff))
                .text_size(px(13.0))
                .text_color(rgb(0x111827))
                .child_any(label)
                .child(
                    NavigationMenuIcon::<&'static str>::new()
                        .text_size(px(10.0))
                        .child(div().child("v")),
                ),
        )
        .child(
            NavigationMenuContent::<&'static str>::new()
                .w(px(content_width))
                .child(div().child(content)),
        )
}

fn tooltip_demo() -> impl IntoElement {
    TooltipProvider::<()>::new().child(basic_tooltip_root(
        "gallery-tooltip-basic",
        "Hover/focus",
        "Basic hover or focus tooltip.",
    ))
}

fn basic_tooltip_root(
    id: &'static str,
    trigger_label: &'static str,
    content: &'static str,
) -> TooltipRoot<()> {
    TooltipRoot::<()>::new()
        .id(id)
        .flex()
        .items_center()
        .gap_2()
        .child(gallery_trigger::<()>(id, trigger_label))
        .child(gallery_popup::<()>(content))
}

fn gallery_trigger<P: Clone + 'static>(id: &'static str, label: &'static str) -> TooltipTrigger<P> {
    TooltipTrigger::<P>::new()
        .id(id)
        .px_3()
        .py_2()
        .rounded_md()
        .border_1()
        .border_color(rgb(0x9ca3af))
        .bg(rgb(0xffffff))
        .text_size(px(13.0))
        .text_color(rgb(0x111827))
        .child(label)
}

fn gallery_popup<P: Clone + 'static>(content: &'static str) -> TooltipPortal<P> {
    TooltipPortal::<P>::new().child(
        TooltipPositioner::<P>::new().child(
            TooltipPopup::<P>::new()
                .rounded_md()
                .bg(rgb(0x111827))
                .text_color(rgb(0xffffff))
                .text_size(px(12.0))
                .py_2()
                .px_3()
                .shadow_lg()
                .child(TooltipViewport::<P>::new().child(content)),
        ),
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

fn combobox_demo() -> impl IntoElement {
    ComboboxRoot::<&'static str>::new()
        .id("gallery-combobox")
        .item_to_string_value(|value| (*value).into())
        .flex()
        .flex_col()
        .gap_2()
        .child(
            ComboboxLabel::new()
                .text_size(px(13.0))
                .child("Fruit search"),
        )
        .child(combobox_input_group())
        .child(combobox_popup_stack("gallery-combobox"))
}

fn multiple_combobox_demo() -> impl IntoElement {
    ComboboxRoot::<&'static str>::new()
        .id("gallery-multiple-combobox")
        .multiple(true)
        .default_values(vec!["apple"])
        .item_to_string_value(|value| (*value).into())
        .flex()
        .flex_col()
        .gap_2()
        .child(
            ComboboxInputGroup::new()
                .w_full()
                .min_h(px(34.0))
                .px_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .bg(rgb(0xffffff))
                .flex()
                .flex_wrap()
                .items_center()
                .gap_1()
                .child(
                    ComboboxChips::new()
                        .flex()
                        .gap_1()
                        .style_with_state(|_state, chips| chips),
                )
                .child(
                    ComboboxInput::new()
                        .id("gallery-multiple-combobox-input")
                        .placeholder("Add fruits…")
                        .flex_1(),
                )
                .child(ComboboxClear::new().text_color(rgb(0x6b7280)))
                .child(
                    ComboboxTrigger::new()
                        .id("gallery-multiple-combobox-trigger")
                        .child(ComboboxIcon::<&'static str>::new().text_size(px(12.0))),
                ),
        )
        .child(combobox_popup_stack("gallery-multiple-combobox"))
}

fn field_combobox_demo() -> impl IntoElement {
    FieldRoot::new()
        .id("gallery-field-combobox")
        .flex()
        .flex_col()
        .gap_2()
        .child(FieldLabel::new().text_size(px(13.0)).child("Fruit"))
        .child_any(
            ComboboxRoot::<&'static str>::new()
                .id("gallery-field-combobox-control")
                .name("fruit")
                .required(true)
                .item_to_string_value(|value| (*value).into())
                .child(combobox_input_group())
                .child(combobox_popup_stack("gallery-field-combobox")),
        )
        .child(
            FieldDescription::new()
                .text_size(px(12.0))
                .child("Type to filter, pick one value."),
        )
        .child(
            FieldError::new()
                .text_size(px(12.0))
                .text_color(rgb(0xb91c1c)),
        )
}

fn combobox_input_group() -> ComboboxInputGroup<&'static str> {
    ComboboxInputGroup::new()
        .w_full()
        .h(px(34.0))
        .px_2()
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .bg(rgb(0xffffff))
        .flex()
        .items_center()
        .gap_1()
        .child(
            ComboboxInput::new()
                .id("gallery-combobox-input")
                .placeholder("Search fruits…")
                .flex_1(),
        )
        .child(ComboboxClear::new().text_color(rgb(0x6b7280)))
        .child(
            ComboboxTrigger::new().id("gallery-combobox-trigger").child(
                ComboboxIcon::<&'static str>::new()
                    .text_size(px(12.0))
                    .text_color(rgb(0x6b7280)),
            ),
        )
}

fn combobox_popup_stack(id_prefix: &'static str) -> ComboboxPortal<&'static str> {
    ComboboxPortal::new().child(
        ComboboxPositioner::new().side_offset(px(4.0)).child(
            ComboboxPopup::new()
                .w(px(220.0))
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .bg(rgb(0xffffff))
                .shadow_lg()
                .p_1()
                .child(
                    ComboboxList::new()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(combobox_item(id_prefix, "apple", "Apple"))
                        .child(combobox_item(id_prefix, "banana", "Banana"))
                        .child(combobox_item(id_prefix, "cherry", "Cherry"))
                        .child(combobox_item(id_prefix, "orange", "Orange"))
                        .child(
                            ComboboxEmpty::new()
                                .px_2()
                                .py_1()
                                .text_size(px(12.0))
                                .text_color(rgb(0x6b7280))
                                .child("No fruits found."),
                        ),
                ),
        ),
    )
}

fn combobox_item(
    id_prefix: &'static str,
    value: &'static str,
    label: &'static str,
) -> ComboboxItem<&'static str> {
    ComboboxItem::new()
        .id(format!("{id_prefix}-item-{value}"))
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
            ComboboxItemIndicator::new()
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
        .child_any(div().text_size(px(13.0)).child(label))
}

fn autocomplete_demo() -> impl IntoElement {
    AutocompleteRoot::<&'static str>::new()
        .id("gallery-autocomplete")
        .item_to_string_value(|value| (*value).into())
        .flex()
        .flex_col()
        .gap_2()
        .child(autocomplete_input_group("gallery-autocomplete"))
        .child(autocomplete_popup_stack("gallery-autocomplete"))
        .child_any(
            div()
                .text_size(px(11.0))
                .text_color(rgb(0x9ca3af))
                .child("Typing filters the list; highlighting never edits the input."),
        )
        .child(
            AutocompleteValue::<&'static str>::new()
                .text_size(px(12.0))
                .text_color(rgb(0x6b7280))
                .formatter(|value| match value.is_empty() {
                    true => "Nothing typed yet.".into(),
                    false => format!("Value: {value}").into(),
                }),
        )
}

fn autocomplete_both_demo() -> impl IntoElement {
    AutocompleteRoot::<&'static str>::new()
        .id("gallery-autocomplete-both")
        .mode(AutocompleteMode::Both)
        .item_to_string_value(|value| (*value).into())
        .flex()
        .flex_col()
        .gap_2()
        .child(autocomplete_input_group("gallery-autocomplete-both"))
        .child(autocomplete_popup_stack("gallery-autocomplete-both"))
        .child_any(div().text_size(px(11.0)).text_color(rgb(0x9ca3af)).child(
            "Type, then press ArrowDown: the highlighted item is written into the input inline.",
        ))
}

fn autocomplete_input_group(id_prefix: &'static str) -> AutocompleteInputGroup<&'static str> {
    AutocompleteInputGroup::new()
        .w_full()
        .h(px(34.0))
        .px_2()
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .bg(rgb(0xffffff))
        .flex()
        .items_center()
        .gap_1()
        .child(
            AutocompleteInput::new()
                .id(format!("{id_prefix}-input"))
                .placeholder("Search fruits…")
                .flex_1(),
        )
        .child(AutocompleteClear::new().text_color(rgb(0x6b7280)))
}

fn autocomplete_popup_stack(id_prefix: &'static str) -> AutocompletePortal<&'static str> {
    AutocompletePortal::new().child(
        AutocompletePositioner::new().side_offset(px(4.0)).child(
            AutocompletePopup::new()
                .w(px(220.0))
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .bg(rgb(0xffffff))
                .shadow_lg()
                .p_1()
                .child(
                    AutocompleteList::new()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(autocomplete_item(id_prefix, "apple", "Apple"))
                        .child(autocomplete_item(id_prefix, "banana", "Banana"))
                        .child(autocomplete_item(id_prefix, "cherry", "Cherry"))
                        .child(autocomplete_item(id_prefix, "orange", "Orange"))
                        .child(
                            AutocompleteEmpty::new()
                                .px_2()
                                .py_1()
                                .text_size(px(12.0))
                                .text_color(rgb(0x6b7280))
                                .child("No fruits found."),
                        ),
                ),
        ),
    )
}

fn autocomplete_item(
    id_prefix: &'static str,
    value: &'static str,
    label: &'static str,
) -> AutocompleteItem<&'static str> {
    AutocompleteItem::new()
        .id(format!("{id_prefix}-item-{value}"))
        .value(value)
        .label(label)
        .px_2()
        .py_1()
        .rounded_sm()
        .style_with_state(|state, item| {
            if state.highlighted {
                item.bg(rgb(0xf3f4f6))
            } else {
                item
            }
        })
        .child_any(div().text_size(px(13.0)).child(label))
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

fn button_demo() -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(gallery_button("example-button", false, false, "Save"))
        .child(gallery_button(
            "example-button-disabled",
            true,
            false,
            "Disabled",
        ))
        .child(gallery_button(
            "example-button-focusable-disabled",
            true,
            true,
            "Focusable disabled",
        ))
}

fn gallery_button(
    id: &'static str,
    disabled: bool,
    focusable_when_disabled: bool,
    label: &'static str,
) -> ButtonRoot {
    ButtonRoot::new()
        .id(id)
        .disabled(disabled)
        .focusable_when_disabled(focusable_when_disabled)
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .px_3()
        .py_2()
        .text_size(px(13.0))
        .bg(rgb(0xffffff))
        .text_color(rgb(0x111827))
        .on_click(|_event, _window, _cx| {
            println!("button clicked");
        })
        .style_with_state(|state, button| {
            let button = if state.disabled {
                button.opacity(0.5)
            } else {
                button.hover(|hovered| hovered.bg(rgb(0xf3f4f6)))
            };

            if state.focused {
                button.border_color(rgb(0x2563eb))
            } else {
                button
            }
        })
        .child(label)
}

fn toggle_demo() -> impl IntoElement {
    div().flex().items_center().gap_2().child(
        Toggle::<gpui::SharedString>::new()
            .id("example-toggle")
            .rounded_md()
            .border_1()
            .border_color(rgb(0xd1d5db))
            .px_3()
            .py_2()
            .text_size(px(13.0))
            .style_with_state(|state, toggle| {
                let toggle = if state.pressed {
                    toggle.bg(rgb(0x111827)).text_color(rgb(0xffffff))
                } else {
                    toggle.bg(rgb(0xffffff)).text_color(rgb(0x111827))
                };

                let toggle = if state.disabled {
                    toggle.opacity(0.5)
                } else {
                    toggle
                };

                if state.focused {
                    toggle.shadow_lg()
                } else {
                    toggle
                }
            })
            .child("Bold"),
    )
}

fn toolbar_demo() -> impl IntoElement {
    ToolbarRoot::new()
        .id("example-toolbar")
        .flex()
        .items_center()
        .gap_1()
        .p_1()
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .child(toolbar_demo_button("toolbar-copy", "Copy", false))
        .child(toolbar_demo_button("toolbar-paste", "Paste", true))
        .child(
            ToolbarGroup::new()
                .style_with_state(|_state, group| group.flex().gap_1())
                .child(toolbar_demo_button("toolbar-cut", "Cut", false))
                .child(
                    ToolbarLink::new()
                        .id("toolbar-help")
                        .px_3()
                        .py_2()
                        .rounded_md()
                        .text_size(px(13.0))
                        .text_color(rgb(0x2563eb))
                        .style_with_state(|state, link| {
                            if state.focused {
                                link.shadow_lg()
                            } else {
                                link
                            }
                        })
                        .child("Help"),
                ),
        )
        .child(
            ToolbarSeparator::new()
                .style_with_state(|_state, separator| separator.w(px(1.0)).h_6().bg(rgb(0xd1d5db))),
        )
        .child(
            ToolbarInput::new()
                .id("toolbar-search")
                .placeholder("Search…")
                .w(px(160.0))
                .px_2()
                .py_1()
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .style_with_state(|state, input| {
                    if state.input.focused {
                        input.border_color(rgb(0x2563eb))
                    } else {
                        input
                    }
                }),
        )
}

fn toolbar_demo_button(id: &'static str, label: &'static str, disabled: bool) -> ToolbarButton {
    ToolbarButton::new()
        .id(id)
        .disabled(disabled)
        .px_3()
        .py_2()
        .rounded_md()
        .text_size(px(13.0))
        .style_with_state(|state, button| {
            let button = if state.disabled {
                button.bg(rgb(0xf3f4f6)).text_color(rgb(0x9ca3af))
            } else {
                button.bg(rgb(0xffffff)).text_color(rgb(0x111827))
            };

            if state.focused {
                button.shadow_lg()
            } else {
                button
            }
        })
        .child(label)
}

fn toggle_group_demo() -> impl IntoElement {
    toggle_group_demo_variant("example-toggle-group", true)
}

fn single_toggle_group_demo() -> impl IntoElement {
    toggle_group_demo_variant("example-toggle-group-single", false)
}

fn toggle_group_demo_variant(id: &'static str, multiple: bool) -> impl IntoElement {
    ToggleGroup::<gpui::SharedString>::new()
        .id(id)
        .multiple(multiple)
        .default_value(vec!["bold".into()])
        .flex()
        .gap_1()
        .p_1()
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .child(gallery_group_toggle(id, "bold", "Bold"))
        .child(gallery_group_toggle(id, "italic", "Italic"))
        .child(gallery_group_toggle(id, "underline", "Underline"))
}

fn gallery_group_toggle(
    id_prefix: &'static str,
    value: &'static str,
    label: &'static str,
) -> Toggle<gpui::SharedString> {
    Toggle::<gpui::SharedString>::new()
        .id(format!("{id_prefix}-{value}"))
        .value(gpui::SharedString::from(value))
        .rounded_md()
        .px_3()
        .py_2()
        .text_size(px(13.0))
        .style_with_state(|state, toggle| {
            let toggle = if state.pressed {
                toggle.bg(rgb(0x111827)).text_color(rgb(0xffffff))
            } else {
                toggle.bg(rgb(0xffffff)).text_color(rgb(0x111827))
            };

            if state.focused {
                toggle.shadow_lg()
            } else {
                toggle
            }
        })
        .child(label)
}

fn collapsible_demo() -> impl IntoElement {
    CollapsibleRoot::new()
        .id("example-collapsible")
        .default_open(true)
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
            CollapsibleTrigger::new()
                .id("example-collapsible-trigger")
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .bg(rgb(0xffffff))
                .px_3()
                .py_2()
                .text_size(px(13.0))
                .style_with_state(|state, trigger| {
                    let trigger = if state.open {
                        trigger.border_color(rgb(0x111827))
                    } else {
                        trigger
                    };

                    if state.focused {
                        trigger.shadow_lg()
                    } else {
                        trigger
                    }
                })
                .child("Toggle details"),
        )
        .child(
            CollapsiblePanel::new()
                .keep_mounted(true)
                .rounded_md()
                .border_1()
                .border_color(rgb(0xe5e7eb))
                .bg(rgb(0xf9fafb))
                .p_3()
                .text_size(px(12.0))
                .text_color(rgb(0x374151))
                .style_with_state(|state, panel| {
                    if state.closed {
                        panel.opacity(0.0)
                    } else {
                        panel.opacity(1.0)
                    }
                })
                .child("Panel content stays mounted and is hidden while closed."),
        )
}

fn accordion_demo() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            AccordionRoot::new()
                .id("example-accordion")
                .default_value(Vec::from(["what"]))
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
                .child(gallery_accordion_item(
                    "what",
                    "What is base_gpui?",
                    "A GPUI-native port of Base UI component behavior.",
                ))
                .child(gallery_accordion_item(
                    "why",
                    "Why Accordion next?",
                    "It builds on Collapsible while adding shared item state.",
                )),
        )
        .child(
            AccordionRoot::new()
                .id("example-accordion-multiple")
                .multiple(true)
                .flex()
                .flex_col()
                .gap_2()
                .child(gallery_accordion_item(
                    "multi-one",
                    "Multiple mode item one",
                    "This item can stay open with its sibling.",
                ))
                .child(gallery_accordion_item(
                    "multi-two",
                    "Multiple mode item two",
                    "Opening this item does not close the first.",
                )),
        )
}

fn gallery_accordion_item(
    value: &'static str,
    trigger_text: &'static str,
    panel_text: &'static str,
) -> AccordionItem<&'static str> {
    AccordionItem::new(value)
        .rounded_md()
        .border_1()
        .border_color(rgb(0xe5e7eb))
        .bg(rgb(0xffffff))
        .style_with_state(|state, item| {
            if state.open {
                item.border_color(rgb(0x111827))
            } else {
                item
            }
        })
        .child(
            AccordionHeader::new().child(
                AccordionTrigger::new()
                    .id(format!("example-accordion-trigger-{value}"))
                    .w_full()
                    .px_3()
                    .py_2()
                    .text_size(px(13.0))
                    .child(trigger_text),
            ),
        )
        .child(
            AccordionPanel::new()
                .px_3()
                .pb_3()
                .text_size(px(12.0))
                .text_color(rgb(0x6b7280))
                .child(panel_text),
        )
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

fn field_otp_field_demo() -> impl IntoElement {
    FieldRoot::new()
        .id("example-otp-field-field")
        .validation_mode(base_gpui::field::FieldValidationMode::OnBlur)
        .flex()
        .flex_col()
        .gap_2()
        .child(
            FieldLabel::new()
                .text_size(px(13.0))
                .text_color(rgb(0x374151))
                .child("One-time code"),
        )
        .child(otp_field_body())
        .child(
            FieldError::new()
                .text_color(rgb(0xdc2626))
                .text_size(px(12.0))
                .child("Code is required."),
        )
}

fn otp_field_body() -> OTPFieldRoot {
    let mut root = OTPFieldRoot::new()
        .id("example-otp-field")
        .name("one-time-code")
        .length(6)
        .required(true)
        .flex()
        .items_center()
        .gap_1();

    for index in 0..6 {
        if index == 3 {
            root = root.child(Separator::new().w(px(10.0)).h(px(2.0)).bg(rgb(0x9ca3af)));
        }
        root = root.child(
            OTPFieldInput::new()
                .w(px(30.0))
                .h(px(36.0))
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd1d5db))
                .style_with_state(|state, slot| {
                    if state.root.disabled {
                        slot.bg(rgb(0xf3f4f6)).opacity(0.6)
                    } else if state.active && state.root.focused {
                        slot.border_color(rgb(0x2563eb))
                    } else if state.filled {
                        slot.bg(rgb(0xf9fafb))
                    } else {
                        slot
                    }
                }),
        );
    }

    root
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

fn toast_demo() -> impl IntoElement {
    // A process-wide manager: demonstrates manager-driven adds from outside
    // the viewport subtree (the buttons live beside, not inside, the stack).
    thread_local! {
        static TOAST_DEMO_MANAGER: ToastManager = create_toast_manager::<()>();
    }
    let manager = TOAST_DEMO_MANAGER.with(ToastManager::clone);

    let toast_button = |id: &'static str, label: &'static str| {
        div()
            .id(id)
            .px_3()
            .py_1p5()
            .rounded_md()
            .bg(rgb(0x1f2937))
            .text_color(rgb(0xf9fafb))
            .text_size(px(13.0))
            .cursor_pointer()
            .child(label)
    };

    let add_manager = manager.clone();
    let upsert_manager = manager.clone();
    let promise_manager = manager.clone();
    let close_manager = manager.clone();

    ToastProvider::new()
        .id("gallery-toast-provider")
        .manager(manager)
        .child_any(
            div()
                .flex()
                .flex_wrap()
                .gap_2()
                .child(
                    toast_button("gallery-toast-add", "Add toast").on_click(move |_, _, cx| {
                        add_manager.add(
                            ToastOptions::new()
                                .title("Saved")
                                .description("Your changes were saved."),
                            cx,
                        );
                    }),
                )
                .child(
                    toast_button("gallery-toast-upsert", "Upsert toast").on_click(
                        move |_, _, cx| {
                            upsert_manager.add(
                                ToastOptions::new()
                                    .id(ToastId::new("gallery-toast-upserted"))
                                    .title("Upserted")
                                    .description("Same id: updated in place, timer reset."),
                                cx,
                            );
                        },
                    ),
                )
                .child(
                    toast_button("gallery-toast-promise", "Promise toast").on_click(
                        move |_, _, cx| {
                            let timer = cx.background_executor().timer(Duration::from_millis(1500));
                            promise_manager
                                .promise(
                                    async move {
                                        timer.await;
                                        Ok::<(), ()>(())
                                    },
                                    ToastPromiseOptions::from_text(
                                        "Uploading…",
                                        "Upload complete",
                                        "Upload failed",
                                    ),
                                    cx,
                                )
                                .detach();
                        },
                    ),
                )
                .child(
                    toast_button("gallery-toast-close-all", "Close all").on_click(
                        move |_, _, cx| {
                            close_manager.close(None, cx);
                        },
                    ),
                ),
        )
        .child(
            ToastPortal::new().child(
                ToastViewport::new()
                    .id("gallery-toast-viewport")
                    .absolute()
                    .bottom_4()
                    .right_4()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .content_builder(|_facts| {
                        ToastRoot::new()
                            .w(px(280.0))
                            .p_3()
                            .rounded_md()
                            .bg(rgb(0x111827))
                            .text_color(rgb(0xf9fafb))
                            .style_with_state(|state, base| {
                                if state.limited {
                                    base.opacity(0.0)
                                } else {
                                    base
                                }
                            })
                            .child(
                                ToastTitle::new()
                                    .text_size(px(13.0))
                                    .font_weight(gpui::FontWeight::SEMIBOLD),
                            )
                            .child(
                                ToastDescription::new()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x9ca3af)),
                            )
                            .child(
                                ToastClose::new()
                                    .absolute()
                                    .top_1()
                                    .right_2()
                                    .cursor_pointer()
                                    .text_size(px(12.0))
                                    .child_any("✕"),
                            )
                    }),
            ),
        )
}

// Overlay composition pattern: a `relative()` container wraps the scrollable
// content, and the scrollbar primitive is overlaid as an absolute sibling
// layer that fills the container.
fn scrollbar_vertical_demo(handle: &ScrollHandle) -> impl IntoElement {
    div()
        .relative()
        .w_full()
        .h(px(120.0))
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .child(
            div()
                .id("scrollbar-vertical-demo")
                .size_full()
                .overflow_y_scroll()
                .track_scroll(handle)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .p_2()
                        .gap_1()
                        .children((0..40).map(|index| {
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0x374151))
                                .child(format!("Row {index}"))
                        })),
                ),
        )
        .child(
            scrollbar_vertical(handle)
                .id("scrollbar-vertical-demo-bar")
                .visibility(ScrollbarVisibility::Scrolling),
        )
}

fn scrollbar_both_axes_demo(handle: &ScrollHandle) -> impl IntoElement {
    div()
        .relative()
        .w_full()
        .h(px(120.0))
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .child(
            div()
                .id("scrollbar-both-demo")
                .size_full()
                .overflow_scroll()
                .track_scroll(handle)
                .child(
                    div()
                        .w(px(700.0))
                        .flex()
                        .flex_col()
                        .p_2()
                        .gap_1()
                        .children((0..40).map(|index| {
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0x374151))
                                .child(format!(
                                    "Row {index} — wide content that overflows horizontally too"
                                ))
                        })),
                ),
        )
        .child(
            scrollbar(handle)
                .id("scrollbar-both-demo-bar")
                .axis(ScrollbarAxis::Both)
                .visibility(ScrollbarVisibility::Always),
        )
}

fn scrollbar_uniform_list_demo(handle: &UniformListScrollHandle) -> impl IntoElement {
    div()
        .relative()
        .w_full()
        .h(px(120.0))
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .child(
            uniform_list("scrollbar-uniform-list-demo", 200, |range, _window, _cx| {
                range
                    .map(|index| {
                        div()
                            .px_2()
                            .text_size(px(12.0))
                            .text_color(rgb(0x374151))
                            .child(format!("Virtualized item {index}"))
                    })
                    .collect()
            })
            .size_full()
            .track_scroll(handle),
        )
        .child(
            scrollbar_vertical(handle)
                .id("scrollbar-uniform-list-demo-bar")
                .visibility(ScrollbarVisibility::Hover),
        )
}

// Base UI's canonical show-on-hover / show-while-scrolling recipe: the
// scrollbar strips fade in from `hovering || scrolling` purely through
// `style_with_state`, while the primitive underneath stays pinned visible.
fn scroll_area_demo() -> impl IntoElement {
    let scrollbar_style = |state: ScrollAreaScrollbarStyleState, strip: Div| {
        if state.hovering || state.scrolling {
            strip.bg(rgb(0xf3f4f6)).opacity(1.0)
        } else {
            strip.opacity(0.0)
        }
    };

    ScrollAreaRoot::new()
        .id("scroll-area-demo")
        .w_full()
        .h(px(140.0))
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd1d5db))
        .child(
            ScrollAreaViewport::new()
                .id("scroll-area-demo-viewport")
                .size_full()
                .child(
                    ScrollAreaContent::new().child(
                        div()
                            .w(px(700.0))
                            .flex()
                            .flex_col()
                            .p_2()
                            .gap_1()
                            .children((0..40).map(|index| {
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x374151))
                                    .child(format!(
                                    "Row {index} — wide scroll area content overflowing both axes"
                                ))
                            })),
                    ),
                ),
        )
        .child(
            ScrollAreaScrollbar::new()
                .id("scroll-area-demo-vertical")
                .orientation(ScrollAreaOrientation::Vertical)
                .style_with_state(scrollbar_style)
                .child(
                    ScrollAreaThumb::new().style_with_state(|_state, style| ScrollbarStyle {
                        thumb_color: hsla(221.0 / 360.0, 0.83, 0.53, 0.6),
                        ..style
                    }),
                ),
        )
        .child(
            ScrollAreaScrollbar::new()
                .id("scroll-area-demo-horizontal")
                .orientation(ScrollAreaOrientation::Horizontal)
                .style_with_state(scrollbar_style)
                .child(
                    ScrollAreaThumb::new().style_with_state(|_state, style| ScrollbarStyle {
                        thumb_color: hsla(221.0 / 360.0, 0.83, 0.53, 0.6),
                        ..style
                    }),
                ),
        )
        .child(ScrollAreaCorner::new().style_with_state(|_state, corner| corner.bg(rgb(0xf3f4f6))))
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
            |_, cx| cx.new(|_| ComponentGallery::new()),
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
        cx.open_window(size(px(1040.0), px(760.0)), |_, _| ComponentGallery::new());
        cx.run_until_parked();
    }
}

use std::{rc::Rc, sync::Arc};

use gpui::{
    div, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::{
    combobox::{
        child_wiring::{ComboboxChildNode, ComboboxChildWiring},
        ComboboxChangeSource, ComboboxChipMoveOutcome, ComboboxContext, ComboboxEscape,
        ComboboxInputStyleState, ComboboxMove, ComboboxMoveNext, ComboboxMovePrevious,
        ComboboxSide, COMBOBOX_KEY_CONTEXT,
    },
    primitives::input::{Input, InputStyleState},
};

type ComboboxInputStyle<T> = Rc<dyn Fn(ComboboxInputStyleState<T>, Div) -> Div + 'static>;
type InnerInputStyle = Rc<dyn Fn(InputStyleState, Div) -> Div + 'static>;

/// The text field: composes the `primitives/input` `Input` (no second
/// text-editing implementation) and only adds Combobox wiring — input value
/// sync, open-on-type, chip navigation hand-off, and key dispatch for list
/// navigation.
///
/// Key composition: text-editing keys stay with `INPUT_KEY_CONTEXT`
/// (printable characters always type; Home/End move the caret and never jump
/// the highlight). ArrowUp/ArrowDown/Escape bubble to the Combobox context;
/// Enter and Backspace-with-chips are handled through key-down observation
/// because the input primitive claims those bindings first.
#[derive(IntoElement)]
pub struct ComboboxInput<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    context: Option<ComboboxContext<T>>,
    placeholder: SharedString,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    style_with_state: Option<ComboboxInputStyle<T>>,
    input_style_with_state: Option<InnerInputStyle>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxInput<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("combobox-input"),
            base: div(),
            context: None,
            placeholder: SharedString::default(),
            disabled: false,
            focus_handle: None,
            style_with_state: None,
            input_style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxInput<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxInput<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| input_focus_handle(&self.id, window, cx));
        let (state, display_value) = context.read(cx, |runtime, props| {
            (
                runtime.input_state(props, ComboboxSide::Bottom),
                runtime.display_value(),
            )
        });
        let disabled = self.disabled || state.root.disabled;
        let read_only = state.root.read_only;
        let open_on_input_click = context.props().open_on_input_click;

        let typed_context = context.clone();
        let click_context = context.clone();
        let key_context_handle = context.clone();
        let next_context = context.clone();
        let previous_context = context.clone();
        let escape_context = context.clone();
        let edge_left_context = context.clone();
        let edge_right_context = context.clone();
        let measure_context = context.clone();

        let mut input = Input::new()
            .id(self.id.clone())
            .value(display_value)
            .placeholder(self.placeholder.clone())
            .disabled(disabled)
            .read_only(read_only)
            .focus_handle(focus_handle)
            .on_value_change_with_context(move |value, window, cx| {
                typed_context.input_typed(value, window, cx);
            })
            .on_edge_left(move |_value, window, cx| {
                // Caret at start: hand ArrowLeft off to chip navigation.
                let has_chips =
                    edge_left_context.read(cx, |runtime, _| !runtime.selected_values().is_empty());
                if !has_chips {
                    return false;
                }
                let outcome = edge_left_context.move_chip_highlight(ComboboxMove::Previous, cx);
                let _ = window;
                !matches!(outcome, ComboboxChipMoveOutcome::NoChips)
            })
            .on_edge_right(move |_value, window, cx| {
                let highlighted = edge_right_context
                    .read(cx, |runtime, _| runtime.highlighted_chip_index().is_some());
                if !highlighted {
                    return false;
                }
                let outcome = edge_right_context.move_chip_highlight(ComboboxMove::Next, cx);
                let _ = window;
                !matches!(outcome, ComboboxChipMoveOutcome::NoChips)
            });
        if let Some(input_style_with_state) = self.input_style_with_state.clone() {
            input = input.style_with_state(move |state, base| input_style_with_state(state, base));
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let wrapper = base
            .id((self.id, "combobox-input-wrapper"))
            .key_context(COMBOBOX_KEY_CONTEXT)
            .on_action(move |_: &ComboboxMoveNext, window, cx| {
                next_context.navigate_list(ComboboxMove::Next, window, cx);
            })
            .on_action(move |_: &ComboboxMovePrevious, window, cx| {
                previous_context.navigate_list(ComboboxMove::Previous, window, cx);
            })
            .on_action(move |_: &ComboboxEscape, window, cx| {
                escape_context.escape_pressed(window, cx);
            })
            .on_key_down(move |event, window, cx| {
                let key = event.keystroke.key.as_str();
                match key {
                    // Enter is claimed by the input primitive's binding, so
                    // activation is observed here: a highlighted item is
                    // selected; with no highlight the popup closes.
                    "enter" => {
                        key_context_handle.activate_highlighted(
                            ComboboxChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    }
                    // Backspace/Delete on a highlighted chip removes it;
                    // Backspace in an empty input removes the last value.
                    "backspace" | "delete" => {
                        if key_context_handle.remove_highlighted_chip(window, cx) {
                            return;
                        }
                        if key == "backspace" {
                            let input_empty = key_context_handle
                                .read(cx, |runtime, _| runtime.input_value().is_empty());
                            if input_empty {
                                key_context_handle.remove_last_value(window, cx);
                            }
                        }
                    }
                    _ => {}
                }
            })
            .on_mouse_down(gpui::MouseButton::Left, move |_event, window, cx| {
                // Pressing the input opens the popup without toggling it
                // closed when `open_on_input_click`.
                if open_on_input_click && !disabled && !read_only {
                    click_context.set_open(
                        true,
                        crate::combobox::ComboboxChangeReason::None,
                        ComboboxChangeSource::Pointer,
                        window,
                        cx,
                    );
                }
            })
            .child(input);

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if measure_context.record_input_bounds(bounds, cx) {
                    window.request_animation_frame();
                }
            })
            .child(wrapper)
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxInput<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_combobox_child(
        mut self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let scoped_id = wiring.scope_child_id(&self.id);
        let focus_handle = input_focus_handle(&scoped_id, window, cx);
        wiring.register_input(focus_handle.clone(), focus_handle.is_focused(window));
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxInput<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxInputStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// Styling hook for the inner input primitive; composes with, not
    /// replaces, the Combobox-level `style_with_state`.
    pub fn input_style_with_state(
        mut self,
        style: impl Fn(InputStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.input_style_with_state = Some(Rc::new(style));
        self
    }
}

fn input_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}

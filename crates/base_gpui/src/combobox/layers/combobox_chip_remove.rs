use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, InteractiveElement as _, IntoElement, MouseButton, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::ComboboxChildNode, ComboboxChangeReason, ComboboxChangeSource,
    ComboboxChipRemoveStyleState, ComboboxContext,
};

/// Removes its chip's value (chip-remove-press reason) and refocuses the
/// input.
#[derive(IntoElement)]
pub struct ComboboxChipRemove<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ComboboxContext<T>>,
    index: Option<usize>,
    style_with_state: Option<Rc<dyn Fn(ComboboxChipRemoveStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxChipRemove<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            index: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for ComboboxChipRemove<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxChipRemove<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxChipRemove<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let index = self.index.unwrap_or(0);
        let state = context.read(cx, |runtime, props| runtime.chip_remove_state(props));
        let disabled = state.disabled;
        let press_context = context.clone();
        let has_custom_children = !self.children.is_empty();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let remove = base.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
            if disabled {
                return;
            }
            press_context.remove_chip(
                index,
                ComboboxChangeReason::ChipRemovePress,
                ComboboxChangeSource::Pointer,
                window,
                cx,
            );
        });

        if has_custom_children {
            remove.children(self.children)
        } else {
            remove.child("×")
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxChipRemove<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxChipRemove<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Selection-order position of the owning chip; assigned by `ComboboxChip`.
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxChipRemoveStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

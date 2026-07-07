use std::{rc::Rc, sync::Arc};

use gpui::{
    div, AnyElement, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::dialog::child_wiring::DialogChildWiring;
use crate::dialog::{
    scoped_dialog_trigger_id, DialogCloseAction, DialogFocusNextAction, DialogFocusPreviousAction,
    DialogOpenChangeReason, DialogOpenChangeSource, DialogPopupStyleState,
    DIALOG_POPUP_KEY_CONTEXT,
};
use crate::drawer::{
    child_wiring::DrawerChildNode, DrawerContext, DrawerPopupChild, DrawerPopupStyleState,
};

type DrawerPopupStyle<P> = Rc<dyn Fn(DrawerPopupStyleState<P>, Div) -> Div + 'static>;

/// The drawer contents container. Inherits the dialog popup focus/Escape
/// behavior and merges drawer facts (drag transform, snap offset, nested and
/// swipe state) into its style state.
#[derive(IntoElement)]
pub struct DrawerPopup<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<DrawerPopupChild<P>>,
    context: Option<DrawerContext<P>>,
    focus_handle: Option<FocusHandle>,
    scoped: bool,
    keep_mounted: bool,
    style_with_state: Option<DrawerPopupStyle<P>>,
}

impl<P: Clone + 'static> Default for DrawerPopup<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("drawer-popup"),
            base: div().relative().occlude(),
            children: Vec::new(),
            context: None,
            focus_handle: None,
            scoped: false,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for DrawerPopup<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DrawerPopup<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| popup_focus_handle(&self.id, window, cx));
        let dialog_state = self
            .context
            .as_ref()
            .map(|context| {
                context
                    .dialog()
                    .read(cx, |runtime, _| runtime.popup_state(self.keep_mounted))
            })
            .unwrap_or_else(|| {
                DialogPopupStyleState::new(
                    false,
                    self.keep_mounted,
                    false,
                    0,
                    None,
                    None,
                    Default::default(),
                )
            });
        let state = match self.context.as_ref() {
            Some(context) => context.read(cx, |runtime, _| {
                DrawerPopupStyleState::from_dialog(dialog_state.clone(), runtime.popup_facts())
            }),
            None => DrawerPopupStyleState::from_dialog(
                dialog_state.clone(),
                crate::drawer::DrawerPopupFacts {
                    expanded: false,
                    nested: false,
                    nested_drawer_count: 0,
                    nested_drawer_swiping: false,
                    nested_swipe_progress: 0.0,
                    swipe_direction: Default::default(),
                    swiping: false,
                    swipe_movement: gpui::Point::default(),
                    snap_point_offset: gpui::Pixels::default(),
                    popup_height: None,
                    frontmost_height: None,
                    swipe_strength: 0.0,
                    swipe_dismissed: false,
                },
            ),
        };
        if !state.mounted {
            return div();
        }

        let close_context = self.context.clone();
        let next_context = self.context.clone();
        let previous_context = self.context.clone();
        let trap_focus = state.modal_mode.traps_focus();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        div().child(
            base.id(self.id)
                .track_focus(&focus_handle.tab_stop(true).tab_index(0))
                .focusable()
                .key_context(DIALOG_POPUP_KEY_CONTEXT)
                .on_action(move |_: &DialogCloseAction, window, cx| {
                    if let Some(context) = close_context.as_ref() {
                        context.dialog().close(
                            DialogOpenChangeReason::EscapeKey,
                            DialogOpenChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    }
                })
                .on_action(move |_: &DialogFocusNextAction, window, cx| {
                    if trap_focus {
                        if let Some(context) = next_context.as_ref() {
                            context.dialog().focus_popup_neighbor(false, window, cx);
                        }
                    } else {
                        window.focus_next(cx);
                    }
                })
                .on_action(move |_: &DialogFocusPreviousAction, window, cx| {
                    if trap_focus {
                        if let Some(context) = previous_context.as_ref() {
                            context.dialog().focus_popup_neighbor(true, window, cx);
                        }
                    } else {
                        window.focus_prev(cx);
                    }
                })
                .children(
                    self.children
                        .into_iter()
                        .map(IntoElement::into_element)
                        .collect::<Vec<AnyElement>>(),
                ),
        )
    }
}

impl<P: Clone + 'static> DrawerChildNode<P> for DrawerPopup<P> {
    fn with_drawer_context(mut self, context: DrawerContext<P>) -> Self {
        if !self.scoped {
            self.id = scoped_dialog_trigger_id(&context.root_id(), &self.id);
        }
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_drawer_context(context.clone()))
            .collect();
        self
    }

    fn wire_drawer_child(
        mut self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.id = wiring.scope_child_id(&self.id);
        let focus_handle = popup_focus_handle(&self.id, window, cx);
        wiring.register_popup_focus_handle(focus_handle.clone());
        self.focus_handle = Some(focus_handle);
        self.scoped = true;
        self.children = self
            .children
            .into_iter()
            .map(|child| child.wire_drawer_child(wiring, window, cx))
            .collect();
        self
    }
}

impl<P: Clone + 'static> DrawerPopup<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<DrawerPopupChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(DrawerPopupChild::Any(child.into_any_element()));
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DrawerPopupStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn popup_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}

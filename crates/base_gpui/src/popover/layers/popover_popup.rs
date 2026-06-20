use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};

use crate::popover::{
    child_wiring::{PopoverChildNode, PopoverChildWiring},
    PopoverAlign, PopoverBoundsKind, PopoverCloseAction, PopoverContext, PopoverOpenChangeReason,
    PopoverOpenChangeSource, PopoverPayloadContentBuilder, PopoverPopupChild,
    PopoverPopupStyleState, PopoverSide, POPOVER_KEY_CONTEXT,
};

#[derive(IntoElement)]
pub struct PopoverPopup<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<PopoverPopupChild<P>>,
    context: Option<PopoverContext<P>>,
    side: PopoverSide,
    align: PopoverAlign,
    keep_mounted: bool,
    payload_content: Option<PopoverPayloadContentBuilder<P>>,
    style_with_state: Option<Rc<dyn Fn(PopoverPopupStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PopoverPopup<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("popover-popup"),
            base: div().relative(),
            children: Vec::new(),
            context: None,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Center,
            keep_mounted: false,
            payload_content: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for PopoverPopup<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverPopup<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.popup_state(self.side, self.align, self.keep_mounted)
                })
            })
            .unwrap_or_else(|| {
                PopoverPopupStyleState::new(false, self.keep_mounted, self.side, self.align, false)
            });
        if !state.mounted {
            return div();
        }

        let context = self.context.clone();
        let close_context = context.clone();
        let measure_context = context.clone();
        let mut children = Vec::new();
        if let Some(payload_content) = self.payload_content {
            let payload = context
                .as_ref()
                .and_then(|context| context.read(cx, |runtime, _| runtime.active_payload()));
            children.push(payload_content(payload.as_ref(), window, cx));
        }
        children.extend(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        );

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(self.id)
        .key_context(POPOVER_KEY_CONTEXT)
        .on_action(move |_: &PopoverCloseAction, window, cx| {
            if let Some(context) = close_context.as_ref() {
                context.close(
                    PopoverOpenChangeReason::EscapeKey,
                    PopoverOpenChangeSource::Keyboard,
                    window,
                    cx,
                );
            }
        })
        .children(children);

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if let Some(context) = measure_context.as_ref() {
                    let changed = context.update(cx, |runtime| {
                        runtime.set_bounds(PopoverBoundsKind::Popup, bounds)
                    });
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(base)
    }
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverPopup<P> {
    fn with_popover_context(mut self, context: PopoverContext<P>) -> Self {
        self.id = ElementId::from((context.root_id(), SharedString::from(self.id.to_string())));
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_popover_context(context.clone()))
            .collect();
        self
    }

    fn wire_popover_child(
        mut self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_popup_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> PopoverPopup<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<PopoverPopupChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(PopoverPopupChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn payload_content(
        mut self,
        content: impl Fn(Option<&P>, &mut Window, &mut App) -> gpui::AnyElement + 'static,
    ) -> Self {
        self.payload_content = Some(Rc::new(content));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverPopupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

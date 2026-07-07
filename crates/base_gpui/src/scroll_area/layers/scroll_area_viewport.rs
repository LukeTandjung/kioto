//! Scroll Area Viewport: the single scroll container. Renders a
//! `div().id(...).overflow_scroll().track_scroll(&handle)` over the
//! runtime's shared [`gpui::ScrollHandle`], observes scroll/overflow facts
//! after layout via `on_children_prepainted`, and joins the tab order only
//! while at least one axis is scrollable.

use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

use gpui::{
    div, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window,
};

use crate::scroll_area::{
    child_wiring::ScrollAreaChildNode, ScrollAreaContext, ScrollAreaRootStyleState,
    ScrollAreaViewportChild,
};

#[derive(IntoElement)]
pub struct ScrollAreaViewport {
    id: ElementId,
    base: Div,
    children: Vec<ScrollAreaViewportChild>,
    style_with_state: Option<Rc<dyn Fn(ScrollAreaRootStyleState, Div) -> Div + 'static>>,
    context: Option<ScrollAreaContext>,
}

impl Default for ScrollAreaViewport {
    fn default() -> Self {
        Self {
            id: ElementId::from("scroll-area-viewport"),
            base: div(),
            children: Vec::new(),
            style_with_state: None,
            context: None,
        }
    }
}

impl Styled for ScrollAreaViewport {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ScrollAreaChildNode for ScrollAreaViewport {
    fn with_scroll_area_context(mut self, context: ScrollAreaContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl RenderOnce for ScrollAreaViewport {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context else {
            // Not wired under a ScrollAreaRoot: render inert content.
            return self.base.id(self.id).children(self.children);
        };

        let handle = context.read(cx, |runtime, _| runtime.scroll_handle());
        let focusable = context.read(cx, |runtime, _| runtime.viewport_focusable());
        let focus_handle = viewport_focus_handle(&self.id, window, cx);

        let style_state = context.read(cx, |runtime, _| runtime.viewport_state());
        // Flex-row + items_start lets a `flex_none` Content take its
        // intrinsic (max-content) width and height instead of being clamped
        // by the viewport — the layout side of `min-width: fit-content`.
        let base = self.base.flex().items_start();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, base),
            None => base,
        };

        let children = self
            .children
            .into_iter()
            .map(|child| child.with_scroll_area_context(context.clone()))
            .collect::<Vec<_>>();

        let observe_context = context.clone();
        let observe_handle = handle.clone();
        let base = if focusable {
            base.track_focus(&focus_handle)
        } else {
            base
        };
        base.on_children_prepainted(move |_bounds, _window, cx| {
            // Post-layout observation: picks up scrolling from any
            // source (wheel, primitive drag/track-click) and layout
            // changes that alter max_offset, notifying only on change.
            let now = Instant::now();
            observe_context.refresh(cx, |runtime, props| {
                let mut changed = runtime.observe_scroll(observe_handle.offset(), now);
                changed |= runtime.refresh_overflow(
                    observe_handle.offset(),
                    observe_handle.max_offset(),
                    props.overflow_edge_threshold(),
                );
                changed
            });
        })
        .id(self.id)
        .overflow_scroll()
        .track_scroll(&handle)
        .children(children)
    }
}

impl ScrollAreaViewport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<ScrollAreaViewportChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<ScrollAreaViewportChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    /// Wrap an arbitrary element as viewport content (Content is optional
    /// for vertical-only use).
    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ScrollAreaViewportChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ScrollAreaRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn viewport_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}

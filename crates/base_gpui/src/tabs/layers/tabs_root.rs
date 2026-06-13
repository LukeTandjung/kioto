use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::{
    api::GenericChild,
    tabs::{
        TabsChild, TabsContext, TabsOrientation, TabsProps, TabsRootRenderState,
        TabsValueChangeHandler,
    },
};

#[derive(IntoElement)]
pub struct TabsRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<TabsChild<T>>,
    default_value: Option<T>,
    value: Option<Option<T>>,
    on_value_change: Option<TabsValueChangeHandler<T>>,
    orientation: TabsOrientation,
    style_with_state: Option<Rc<dyn Fn(TabsRootRenderState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for TabsRoot<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("tabs"),
            base: div(),
            children: Vec::from([]),
            default_value: None,
            value: None,
            on_value_change: None,
            orientation: TabsOrientation::Horizontal,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsRoot<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsRoot<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_controlled = self.value.is_some();
        let orientation = self.orientation;
        let context = TabsContext::new(
            self.id,
            cx,
            window,
            self.value,
            self.default_value,
            TabsProps::new(orientation, self.on_value_change),
        );

        context.update(cx, |runtime| runtime.clear_registered_metadata());

        for child in &self.children {
            child.register_runtime(&context, window, cx);
        }

        if !is_controlled {
            context.update(cx, |runtime| runtime.apply_fallback());
        }

        context.update(cx, |runtime| {
            runtime.sync_activation_direction_with_selected_value(orientation);
            runtime.sync_highlighted_tab_with_selected_value();
        });

        // Scoped tab key bindings only dispatch once focus is inside the tabs list.
        // Seed focus once from the synced highlighted tab so initial keyboard use works.
        let focus_handle = context.update(cx, |runtime| runtime.take_initial_focus_handle());
        if let Some(focus_handle) = focus_handle {
            focus_handle.focus(window, cx);
        }

        let render_state =
            context.read(cx, |runtime, props| runtime.root_state(props.orientation()));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(render_state, self.base),
            None => self.base,
        };
        base.children(
            self.children
                .into_iter()
                .map(|child| child.add_state_context(context.clone())),
        )
    }
}

impl<T: Clone + Eq + 'static> TabsRoot<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<TabsChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<TabsChild<T>>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn value(mut self, value: Option<T>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, default_value: Option<T>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(Option<&T>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TabsRootRenderState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

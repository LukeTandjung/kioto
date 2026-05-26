use std::rc::Rc;

use gpui::{
    App, Div, ElementId, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
    div,
};

use crate::{
    api::GenericChild,
    tabs::{TabsChild, TabsContext, TabsOrientation, TabsProps},
};

#[derive(IntoElement)]
pub struct TabsRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<TabsChild<T>>,
    default_value: Option<T>,
    value: Option<Option<T>>,
    on_value_change: Option<Rc<dyn Fn(Option<&T>, &mut Window, &mut App) + 'static>>,
    orientation: TabsOrientation,
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
        let context = TabsContext::new(
            self.id.clone(),
            cx,
            window,
            self.value,
            self.default_value,
            TabsProps::new(self.orientation, self.on_value_change),
        );

        context.clear_registered_metadata(cx);

        let mut registered_panel_index = 0;

        for child in &self.children {
            child.register_runtime(&mut registered_panel_index, &context, cx);
        }

        context.apply_automatic_fallback(cx);
        context.sync_activation_direction_with_selected_value(cx);
        context.sync_highlighted_tab_with_selected_value(cx);

        let mut panel_index = 0;

        self.base.children(
            self.children
                .into_iter()
                .map(|child| {
                    child
                        .map_panel(|panel| {
                            let panel = panel.index(panel_index);
                            panel_index += 1;
                            panel
                        })
                        .add_state_context(context.clone())
                }),
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
}

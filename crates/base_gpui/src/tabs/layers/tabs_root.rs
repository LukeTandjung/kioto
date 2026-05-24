use std::rc::Rc;

use gpui::{
    App, ClickEvent, Div, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div,
};

use crate::{
    tabs::{TabsChild, TabsState},
    api::GenericChild, utils::ControlledState,
};

pub struct TabsRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<TabsChild<T>>,
    default_value: Option<T>,
    value: Option<Option<T>>,
    on_value_change: Option<Rc<dyn Fn(Option<&T>, &ClickEvent, &mut Window, &mut App) + 'static>>,
    orientation: Option<SharedString>,
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
            orientation: Some(SharedString::from("horizontal")),
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
        let state = ControlledState::<TabsState<T>>::new(
            self.id.clone(),
            cx,
            window,
            self.value,
            self.default_value,
        );
        let _value = state.get_state(cx);

        self.base.children(self.children.into_iter().map(|child| {
            child.add_state_context(state.clone())
        }))
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

    pub fn on_value_change(mut self, on_value_change: Rc<dyn Fn(Option<&T>, &ClickEvent, &mut Window, &mut App) + 'static>) -> Self {
        self.on_value_change = Some(on_value_change);
        self     
    }

    pub fn orientation(mut self, orientation: impl Into<SharedString>) -> Self {
        self.orientation = Some(orientation.into());
        self
    }
}

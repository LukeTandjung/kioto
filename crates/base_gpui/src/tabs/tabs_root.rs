use std::rc::Rc;
use gpui::{
    App,
    RenderOnce,
    Window,
    IntoElement,
    SharedString,
    StyleRefinement,
    ParentElement,
    AnyElement,
    Styled,
    Div,
    ClickEvent,
    div
};

pub struct TabsRoot<T: 'static> {
    base: Div,
    children: Vec<AnyElement>,
    default_value: Option<T>,
    value: Option<T>,
    on_value_change: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    orientation: Option<SharedString>,
}

impl<T> Default for TabsRoot<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            default_value: None,
            value: None,
            on_value_change: None,
            orientation: Some(SharedString::from("horizontal")),
        }
    }
}

impl<T> ParentElement for TabsRoot<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T> Styled for TabsRoot<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T> RenderOnce for TabsRoot<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

impl<T> TabsRoot<T> {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn default_value(mut self, default_value: T) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn on_value_change(mut self, on_value_change: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>) -> Self {
        self.on_value_change = Some(on_value_change);
        self     
    }

    pub fn orientation(mut self, orientation: impl Into<SharedString>) -> Self {
        self.orientation = Some(orientation.into());
        self
    }
}

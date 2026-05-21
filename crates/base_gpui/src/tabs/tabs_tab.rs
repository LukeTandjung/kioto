use gpui::{
    App,
    RenderOnce,
    Window,
    IntoElement,
    StyleRefinement,
    ParentElement,
    AnyElement,
    Styled,
    Div,
    div
};

pub struct TabsTab<T: 'static> {
    base: Div,
    children: Vec<AnyElement>,
    value: Option<T>,
    disabled: bool,    
}

impl<T> Default for TabsTab<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            value: None,
            disabled: false
        }
    }
}

impl<T> ParentElement for TabsTab<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);    
    }
}

impl<T> Styled for TabsTab<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T> RenderOnce for TabsTab<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

impl<T> TabsTab<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

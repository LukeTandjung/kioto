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

pub struct TabsPanel<T: 'static> {
    base: Div,
    children: Vec<AnyElement>,
    value: Option<T>,
    keep_mounted: bool,    
}

impl<T> Default for TabsPanel<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            value: None,
            disabled: false
        }
    }
}

impl<T> ParentElement for TabsPanel<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);    
    }
}

impl<T> Styled for TabsPanel<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T> RenderOnce for TabsPanel<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

impl<T> TabsPanel<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }
}

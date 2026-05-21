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

pub struct TabsIndicator {
    base: Div,
    children: Vec<AnyElement>
}

impl Default for TabsIndicator {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([])
        }
    }
}

impl ParentElement for TabsIndicator {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);    
    }
}

impl Styled for TabsIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for TabsIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

impl TabsIndicator {
    pub fn new() -> Self {
        Self::default()
    }
}

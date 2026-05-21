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

pub struct TabsList {
    base: Div,
    children: Vec<AnyElement>,
    activate_on_focus: bool,
    loop_focus: bool
}

impl Default for TabsList {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            activate_on_focus: false,
            loop_focus: true
        }
    }
}

impl ParentElement for TabsList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);    
    }
}

impl Styled for TabsList {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for TabsList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

impl TabsList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn activate_on_focus(mut self, activate_on_focus: bool) -> Self {
        self.activate_on_focus = activate_on_focus;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }
}

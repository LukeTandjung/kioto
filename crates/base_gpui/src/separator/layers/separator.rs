use std::rc::Rc;

use gpui::{div, App, Div, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::separator::{SeparatorOrientation, SeparatorStyleState};

#[derive(IntoElement)]
pub struct Separator {
    base: Div,
    orientation: SeparatorOrientation,
    style_with_state: Option<Rc<dyn Fn(SeparatorStyleState, Div) -> Div + 'static>>,
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            base: div(),
            orientation: SeparatorOrientation::default(),
            style_with_state: None,
        }
    }
}

impl Styled for Separator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let state = SeparatorStyleState::new(self.orientation);

        match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
    }
}

impl Separator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn horizontal(self) -> Self {
        self.orientation(SeparatorOrientation::Horizontal)
    }

    pub fn vertical(self) -> Self {
        self.orientation(SeparatorOrientation::Vertical)
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SeparatorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

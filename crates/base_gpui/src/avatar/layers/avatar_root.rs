use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::avatar::{
    child_wiring::wire_children, AvatarChild, AvatarContext, AvatarRootStyleState,
};

#[derive(IntoElement)]
pub struct AvatarRoot {
    id: ElementId,
    base: Div,
    children: Vec<AvatarChild>,
    style_with_state: Option<Rc<dyn Fn(AvatarRootStyleState, Div) -> Div + 'static>>,
}

impl Default for AvatarRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("avatar"),
            base: div(),
            children: Vec::new(),
            style_with_state: None,
        }
    }
}

impl Styled for AvatarRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for AvatarRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = AvatarContext::new(self.id.clone(), cx, window);
        let wired_children = wire_children(self.children, context.clone());

        context.update(cx, |runtime| runtime.sync_image(wired_children.image_key));

        let style_state = context.read(cx, |runtime| runtime.root_state());
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(self.id).children(wired_children.children)
    }
}

impl AvatarRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<AvatarChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<AvatarChild>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_element(mut self, element: impl IntoElement) -> Self {
        self.children
            .push(AvatarChild::Any(element.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(AvatarRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

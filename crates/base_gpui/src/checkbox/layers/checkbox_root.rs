use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::{
    api::GenericChild,
    checkbox::{
        CheckboxCheckedChangeHandler, CheckboxChild, CheckboxContext, CheckboxProps,
        CheckboxRootRenderState,
    },
};

#[derive(IntoElement)]
pub struct CheckboxRoot {
    id: ElementId,
    base: Div,
    children: Vec<CheckboxChild>,
    default_checked: bool,
    checked: Option<bool>,
    indeterminate: bool,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_checked_change: Option<CheckboxCheckedChangeHandler>,
    style_with_state: Option<Rc<dyn Fn(CheckboxRootRenderState, Div) -> Div + 'static>>,
}

impl Default for CheckboxRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("checkbox"),
            base: div(),
            children: Vec::new(),
            default_checked: false,
            checked: None,
            indeterminate: false,
            disabled: false,
            read_only: false,
            required: false,
            on_checked_change: None,
            style_with_state: None,
        }
    }
}

impl Styled for CheckboxRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for CheckboxRoot {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children
            .extend(elements.into_iter().map(CheckboxChild::from));
    }
}

impl RenderOnce for CheckboxRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = CheckboxContext::new(
            self.id,
            cx,
            window,
            self.checked.map(Some),
            Some(self.default_checked),
            CheckboxProps::new(
                self.indeterminate,
                self.disabled,
                self.read_only,
                self.required,
                self.on_checked_change,
            ),
        );

        let render_state = context.root_render_state(cx);
        let base = match self.style_with_state {
            Some(style) => style(render_state, self.base),
            None => self.base,
        };

        base.children(
            self.children
                .into_iter()
                .map(|child| child.add_state_context(context.clone())),
        )
    }
}

impl CheckboxRoot {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn child(mut self, child: impl Into<CheckboxChild>) -> Self {
        self.children.push(child.into());
        self
    }
    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<CheckboxChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    pub fn default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }
    pub fn checked(mut self, checked: Option<bool>) -> Self {
        self.checked = checked;
        self
    }
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    pub fn on_checked_change(
        mut self,
        on_checked_change: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_change = Some(Rc::new(on_checked_change));
        self
    }
    pub fn style_with_state(
        mut self,
        style: impl Fn(CheckboxRootRenderState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

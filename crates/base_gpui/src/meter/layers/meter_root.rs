use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::meter::{MeterChild, MeterContext, MeterProps, MeterStyleState};

#[derive(IntoElement)]
pub struct MeterRoot {
    id: ElementId,
    base: Div,
    children: Vec<MeterChild>,
    value: f64,
    min: f64,
    max: f64,
    format: Option<Rc<dyn Fn(f64) -> String + 'static>>,
    style_with_state: Option<Rc<dyn Fn(MeterStyleState, Div) -> Div + 'static>>,
}

impl Default for MeterRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("meter"),
            base: div(),
            children: Vec::from([]),
            value: 0.0,
            min: 0.0,
            max: 100.0,
            format: None,
            style_with_state: None,
        }
    }
}

impl Styled for MeterRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for MeterRoot {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let props = MeterProps::new(self.value, self.min, self.max, self.format);
        let context = MeterContext::new(self.id.clone(), &props);

        let children: Vec<AnyElement> = self
            .children
            .into_iter()
            .map(|child| match child {
                MeterChild::Track(track) => {
                    track.with_meter_context(context.clone()).into_any_element()
                }
                MeterChild::Value(value) => {
                    value.with_meter_context(context.clone()).into_any_element()
                }
                MeterChild::Label(label) => {
                    label.with_meter_context(context.clone()).into_any_element()
                }
                MeterChild::Any(any) => any,
            })
            .collect();

        let style_state = context.read(|runtime| runtime.state());
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(self.id).children(children)
    }
}

impl MeterRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<MeterChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<MeterChild>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MeterChild::Any(child.into_any_element()));
        self
    }

    /// The current value of the meter (Base UI's required `value`).
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Custom formatter receiving the raw (unclamped) value.
    pub fn format(mut self, format: impl Fn(f64) -> String + 'static) -> Self {
        self.format = Some(Rc::new(format));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MeterStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

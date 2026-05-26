use std::rc::Rc;

use gpui::{
    App, Div, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
};

use crate::{
    api::GenericChild,
    tabs::{
        TabsActivateHighlighted, TabsContext, TabsListRenderState, TabsOrientation,
        TabsSelectDown, TabsSelectFirst, TabsSelectLast, TabsSelectLeft, TabsSelectRight, TabsSelectUp,
        TABS_LIST_KEY_CONTEXT,
    },
};

use super::TabsTab;

#[derive(IntoElement)]
pub struct TabsList<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<TabsTab<T>>,
    context: Option<TabsContext<T>>,
    activate_on_focus: bool,
    loop_focus: bool,
    style_with_state: Option<Rc<dyn Fn(TabsListRenderState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for TabsList<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            context: None,
            activate_on_focus: false,
            loop_focus: true,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsList<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self.context;
        let render_state = context
            .as_ref()
            .map(|context| context.list_render_state(_cx));
        let bounds_context = context.clone();
        let select_left_context = context.clone();
        let select_right_context = context.clone();
        let select_up_context = context.clone();
        let select_down_context = context.clone();
        let select_first_context = context.clone();
        let select_last_context = context.clone();
        let activate_context = context.clone();
        let activate_on_focus = self.activate_on_focus;
        let loop_focus = self.loop_focus;

        let base = match (self.style_with_state, render_state) {
            (Some(style_with_state), Some(render_state)) => style_with_state(render_state, self.base),
            _ => self.base,
        };

        base
            .on_children_prepainted(move |bounds, _window, cx| {
                if let Some(context) = bounds_context.as_ref() {
                    context.register_tab_bounds(bounds, cx);
                }
            })
            .id("tabs-list")
            .key_context(TABS_LIST_KEY_CONTEXT)
            .focusable()
            .on_action(move |_: &TabsSelectLeft, window, cx| {
                let Some(context) = select_left_context.as_ref() else {
                    return;
                };

                if context.props().orientation() != TabsOrientation::Horizontal {
                    return;
                }

                context.highlight_previous_tab(loop_focus, cx);

                if activate_on_focus {
                    context.select_highlighted_tab(window, cx);
                }
            })
            .on_action(move |_: &TabsSelectRight, window, cx| {
                let Some(context) = select_right_context.as_ref() else {
                    return;
                };

                if context.props().orientation() != TabsOrientation::Horizontal {
                    return;
                }

                context.highlight_next_tab(loop_focus, cx);

                if activate_on_focus {
                    context.select_highlighted_tab(window, cx);
                }
            })
            .on_action(move |_: &TabsSelectUp, window, cx| {
                let Some(context) = select_up_context.as_ref() else {
                    return;
                };

                if context.props().orientation() != TabsOrientation::Vertical {
                    return;
                }

                context.highlight_previous_tab(loop_focus, cx);

                if activate_on_focus {
                    context.select_highlighted_tab(window, cx);
                }
            })
            .on_action(move |_: &TabsSelectDown, window, cx| {
                let Some(context) = select_down_context.as_ref() else {
                    return;
                };

                if context.props().orientation() != TabsOrientation::Vertical {
                    return;
                }

                context.highlight_next_tab(loop_focus, cx);

                if activate_on_focus {
                    context.select_highlighted_tab(window, cx);
                }
            })
            .on_action(move |_: &TabsSelectFirst, window, cx| {
                let Some(context) = select_first_context.as_ref() else {
                    return;
                };

                context.highlight_first_tab(cx);

                if activate_on_focus {
                    context.select_highlighted_tab(window, cx);
                }
            })
            .on_action(move |_: &TabsSelectLast, window, cx| {
                let Some(context) = select_last_context.as_ref() else {
                    return;
                };

                context.highlight_last_tab(cx);

                if activate_on_focus {
                    context.select_highlighted_tab(window, cx);
                }
            })
            .on_action(move |_: &TabsActivateHighlighted, window, cx| {
                let Some(context) = activate_context.as_ref() else {
                    return;
                };

                context.select_highlighted_tab(window, cx);
            })
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(index, tab)| {
                        match context.clone() {
                            Some(context) => tab
                                .index(index)
                                .add_state_context(context)
                                .into_any_element(),
                            None => tab
                                .index(index)
                                .into_any_element(),
                        }
                    })
            )
    }
}

impl<T: Clone + Eq + 'static>
    GenericChild<TabsContext<T>> for TabsList<T>
{
    fn add_state_context(
        mut self,
        context: TabsContext<T>,
    ) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> TabsList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, tab: TabsTab<T>) -> Self {
        self.children.push(tab);
        self
    }

    pub fn children(mut self, tabs: impl IntoIterator<Item = TabsTab<T>>) -> Self {
        self.children.extend(tabs);
        self
    }

    pub fn activate_on_focus(mut self, activate_on_focus: bool) -> Self {
        self.activate_on_focus = activate_on_focus;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TabsListRenderState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    pub fn register_runtime(&self, context: &TabsContext<T>, cx: &mut App) {
        for (index, tab) in self.children.iter().enumerate() {
            tab.register_runtime(index, context, cx);
        }
    }
}

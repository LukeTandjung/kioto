use gpui::{App, ElementId, Window};

use crate::{
    api::GenericContext,
    checkbox::{
        CheckboxIndicatorRenderState, CheckboxProps, CheckboxRootRenderState, CheckboxRuntime,
        CheckboxState,
    },
};

#[derive(Clone)]
pub struct CheckboxContext {
    inner: GenericContext<CheckboxState, CheckboxProps, CheckboxRuntime>,
}

impl CheckboxContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<bool>>,
        default: Option<bool>,
        props: CheckboxProps,
    ) -> Self {
        Self {
            inner: GenericContext::new(
                id,
                cx,
                window,
                controlled,
                default,
                props,
                CheckboxRuntime::new(),
            ),
        }
    }

    pub fn checked(&self, cx: &App) -> bool {
        self.inner.get_state(cx).unwrap_or(false)
    }

    pub fn request_toggle(&self, window: &mut Window, cx: &mut App) {
        if self.inner.props().disabled() || self.inner.props().read_only() {
            return;
        }

        let next = !self.checked(cx);
        self.inner.set_state(Some(next), cx, |props, value, cx| {
            if let (Some(on_checked_change), Some(value)) = (props.on_checked_change(), value) {
                on_checked_change(*value, window, cx);
            }
        });
    }

    pub fn root_render_state(&self, cx: &App) -> CheckboxRootRenderState {
        let props = self.inner.props();
        CheckboxRootRenderState::new(
            self.checked(cx),
            props.disabled(),
            props.read_only(),
            props.required(),
            props.indeterminate(),
            self.inner.get_runtime(cx, |runtime| runtime.focused()),
        )
    }

    pub fn sync_focused(&self, focused: bool, cx: &mut App) {
        self.inner
            .set_runtime_if_changed(cx, |runtime| match runtime.focused() == focused {
                true => false,
                false => {
                    runtime.set_focused(focused);
                    true
                }
            });
    }

    pub fn indicator_render_state(
        &self,
        keep_mounted: bool,
        cx: &App,
    ) -> CheckboxIndicatorRenderState {
        CheckboxIndicatorRenderState::new(self.root_render_state(cx), keep_mounted)
    }
}

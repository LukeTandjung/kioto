use gpui::IntoElement;

pub trait GenericChild<S>: IntoElement {
    fn add_state_context(self, state: S) -> Self;
}

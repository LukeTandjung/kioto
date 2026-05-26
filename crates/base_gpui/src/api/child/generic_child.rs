use gpui::IntoElement;

pub trait GenericChild<C>: IntoElement {
    fn add_state_context(self, context: C) -> Self;
}

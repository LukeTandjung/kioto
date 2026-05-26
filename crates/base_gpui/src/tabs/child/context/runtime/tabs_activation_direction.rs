#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TabsActivationDirection {
    #[default]
    None,
    Left,
    Right,
    Up,
    Down,
}

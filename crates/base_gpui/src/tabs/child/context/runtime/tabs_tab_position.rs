use gpui::Pixels;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabsTabPosition {
    pub left: Pixels,
    pub right: Pixels,
    pub top: Pixels,
    pub bottom: Pixels,
}

impl TabsTabPosition {
    pub fn new(left: Pixels, right: Pixels, top: Pixels, bottom: Pixels) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

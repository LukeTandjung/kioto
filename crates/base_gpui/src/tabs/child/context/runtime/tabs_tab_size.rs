use gpui::Pixels;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabsTabSize {
    pub width: Pixels,
    pub height: Pixels,
}

impl TabsTabSize {
    pub fn new(width: Pixels, height: Pixels) -> Self {
        Self { width, height }
    }
}

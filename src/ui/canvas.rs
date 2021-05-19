
use super::rect::Rect;

pub struct Canvas {
    size : Rect
}
impl Canvas {
    pub fn new(size : Rect) -> Self {
        Self {
            size,
        }
    }
}
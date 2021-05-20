#[derive(Clone)]
pub struct Rect {
    width : u16,
    height : u16,
    x : u16,
    y : u16,
}

impl Rect {
    pub fn new(x : u16, y : u16, width : u16, height : u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
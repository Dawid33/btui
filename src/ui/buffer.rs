pub use crate::ui::prelude::*;

#[derive(Clone)]
pub struct Buffer {
    pub size : Rect,
    pub content : Vec<String>,
}
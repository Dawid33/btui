pub use crate::ui::prelude::*;

#[derive(Clone)]
pub struct Block {
    style : Style,
}

impl Widget for Block {
    fn draw(&self, buffer : &mut Buffer) {
        buffer.content.push("Hello World!".to_string());
    }
}
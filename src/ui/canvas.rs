
use crate::ui::prelude::*;

#[derive(Clone)]
pub struct Canvas {
    pub size : Rect,
    pub canvas_list : Vec<Canvas>,
}

impl Canvas {
    pub fn new(size : Rect) -> Self {
        Self {
            size,
            canvas_list : Vec::new(),
        }
    }
    ///This function just draws what you want onto the terminal.
    ///Slightly faster than draw_safe(), at the cost of zero error handling.
    pub fn draw<W: Widget>(&self, widget : W, rect : Rect) {
        
    }
    pub fn draw_safe<W: Widget>(&self, widget : W, rect : Rect) -> Result<(), DrawError> {
        Ok(())
    }
}
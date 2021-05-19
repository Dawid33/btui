mod event;

use crate::backend;
use crate::ui;
use ui::prelude::*;

pub struct Terminal<B : backend::Backend> {
    pub backend : B,
    pub canvas : Canvas,
}

impl<B> Terminal<B> 
where 
    B : std::fmt::Debug + backend::Backend
{
    pub fn new(backend : B) -> Self {
        Self {
            canvas : Canvas::new(backend.size().expect("Cannot get size of the terminal from the backend.")),
            backend,
        }
    }
    pub fn draw(&self, canvas : &Canvas) {
        
    }
    pub fn size(&self) -> Rect {
        if let Ok(size) = self.backend.size() {
            size
        } else {
            panic!("Cannot get size from backend {:?}", &self.backend)
        }
    }
}
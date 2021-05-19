use super::backend;
use super::ui;

pub struct Terminal<B : backend::Backend> {
    backend : B,
    canvas : ui::Canvas,
}

impl<B : backend::Backend> Terminal<B> {
    pub fn new(backend : B) -> Self {
        Self {
            canvas : ui::Canvas::new(backend.size().expect("Cannot get size of the terminal from the backend.")),
            backend,
        }
    }
} 
use super::backend::*;
use std::io::Write;
pub struct Terminal<B : Backend> {
    backend : B
}

impl<B : Backend> Terminal<B> {
    pub fn new(backend : B) -> Self {
        Self {
            backend,
        }
    }
} 
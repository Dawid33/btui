#![allow(unused_imports)]
#![allow(dead_code)]

pub mod backend;
pub mod terminal;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::terminal::*;
    use crate::backend::*;
    use crate::ui::prelude::*;
    #[test]
    fn it_works() {
        let terminal = Terminal::new(TermionBackend::new());
        
        let base = Canvas::new(terminal.size());
        terminal.draw(&base);
    }
}

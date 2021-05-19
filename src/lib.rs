mod backend;
mod terminal;
mod ui;


#[cfg(test)]
mod tests {
    use crate::terminal::*;
    use crate::backend::*;
    #[test]
    fn it_works() {
        let term = Terminal::new(TermionBackend::new());
    }
}

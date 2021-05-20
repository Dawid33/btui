use crate::ui::prelude::*;

pub trait Widget {
    ///Draw the current state of the widget onto the buffer.
    fn draw(&self, buffer : &mut Buffer);
}

/* Secret code in case things get hairy.
pub trait CloneWidget {
    fn clone_foo<'a>(&self) -> Box<dyn Widget>;
}

impl<T> CloneWidget for T
where
    T: Widget + Clone + 'static,
{
    fn clone_foo(&self) -> Box<dyn Widget> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Widget> {
    fn clone(&self) -> Self {
        self.clone_foo()
    }
}
*/
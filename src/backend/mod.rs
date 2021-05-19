#[cfg(feature = "termion")]
pub mod termion;
#[cfg(feature = "termion")]
pub use self::termion::TermionBackend;

use std::io;
use crate::ui::prelude::*;
pub trait Backend : Clone + io::Write{
    fn write(&mut self, buf : &[u8]) -> io::Result<()>;
    fn flush(&mut self) -> io::Result<()>;
    fn size(&self) -> io::Result<Rect>;
}
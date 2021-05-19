#[cfg(feature = "termion")]
pub mod termion;
#[cfg(feature = "termion")]
pub use self::termion::TermionBackend;

pub trait Backend : Clone {
    fn write(&mut self, buf : &[u8]) -> std::io::Result<()>;
    fn flush(&mut self, buf : &[u8]) -> std::io::Result<()>;
}
use std::io::{self, Write};

pub struct TermionBackend(std::io::Stdout);

impl Clone for TermionBackend {
    fn clone(&self) -> Self {
        Self(std::io::stdout())
    }
}
impl TermionBackend {
    pub fn new() -> TermionBackend {
        TermionBackend(std::io::stdout())
    }
}
impl Write for TermionBackend {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
impl super::Backend for TermionBackend {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let result = self.0.write_all(buf);
        result
    }
}

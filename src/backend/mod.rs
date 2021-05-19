pub mod termion;
use std::io::Write;

pub enum Backend<W : Write> {
    Termion(W),
}
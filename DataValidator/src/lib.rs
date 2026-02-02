use eyre::Result;
use std::error::Error;

mod input;
mod parsers;

pub fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub use input::{ArgsInner, ByteStream, FileFormatInner};

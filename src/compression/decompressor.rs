use super::Method;
use dh::{Readable, Writable};
use std::io::{Error, ErrorKind, Result};

pub fn decompress<'a>(
    reader: &mut dyn Readable<'a>,
    offset: u64,
    size: u64,
    method: &Method,
    target: &mut dyn Writable<'a>,
    buffer_size: u64,
) -> Result<()> {
    use Method::*;
    match method {
        Stored => reader.copy_at(offset, size, target, buffer_size),
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            "Unsupported compression method",
        )),
    }
}

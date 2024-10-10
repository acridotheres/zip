use crate::{compression::decompressor::decompress, File};
use dh::{Readable, Writable};
use std::io::Result;

pub fn get_content(reader: &mut dyn Readable, file: &File, buffer_size: u64) -> Result<Vec<u8>> {
    let mut target = dh::data::write_new(file.uncompressed_size);
    decompress(
        reader,
        file.offset,
        file.size,
        &file.compression,
        &mut target,
        buffer_size,
    )?;
    Ok(dh::data::close(target))
}

pub fn extract_content<'a>(
    reader: &mut dyn Readable<'a>,
    target: &mut dyn Writable<'a>,
    file: &File,
    buffer_size: u64,
) -> Result<()> {
    decompress(
        reader,
        file.offset,
        file.size,
        &file.compression,
        target,
        buffer_size,
    )
}

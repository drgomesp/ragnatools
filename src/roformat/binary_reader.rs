use std::io::{Error, Read};
use std::path::Path;

use encoding;
use encoding::DecoderTrap;
use encoding::types::Encoding;

pub struct BinaryReader {
    buf: Vec<u8>,
    index: usize,
}

impl BinaryReader {
    pub fn new<R: Read>(mut reader: R) -> Result<BinaryReader, Error> {
        let mut _reader = BinaryReader {
            buf: Vec::new(),
            index: 0,
        };

        let _ = reader.read_to_end(&mut _reader.buf)?;

        return Ok(_reader);
    }

    pub fn skip(&mut self, size: u32) {
        self.index += size as usize;
    }

    pub fn string(&mut self, max_len: u32) -> String {
        let i = self.index;
        self.index += max_len as usize;

        let bytes: Vec<u8> = self
            .buf
            .iter()
            .skip(i)
            .take(max_len as usize)
            .take_while(|b| **b != 0)
            .map(|b| *b)
            .collect();

        let decoded = encoding::all::WINDOWS_1252
            .decode(&bytes, DecoderTrap::Strict).unwrap();

        decoded
    }

    pub fn next_u32(&mut self) -> u32 {
        let result = unsafe { *(self.buf.as_ptr().offset(self.index as isize) as *const u32) };
        self.index += 4;
        return result;
    }
}

use std::{fmt, mem};
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::fmt::{Error as StdError, Formatter};
use std::io::{BufReader, Read, SeekFrom};
use std::slice::from_raw_parts_mut;
use std::str::from_utf8;
use std::string::FromUtf8Error;

use bitflags::bitflags;

use crate::roformat::binary_reader::BinaryReader;
use crate::roformat::grf::entry::Entry;
use crate::roformat::grf::error::GrfError;

pub mod entry;
pub mod error;

const HEADER_SIZE: u32 = 15 + 15 + 4 * 4;
const HEADER_SIGNATURE: &str = "Master of Magic";

#[derive(Debug, Default)]
// NOTE: @see https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
// We want this to behave just like in C in terms of order size and field alignment.
pub struct GrfHeader {
    signature: String,
    encryption_key: String,
    file_table_offset: u32,
    entry_count: u32,
    reserved_files: u32,
    version: u32,
}

#[derive(Debug, Default)]
pub struct GrfFile {
    header: GrfHeader,
    entries: Vec<Entry>,
}

impl GrfFile {
    pub fn parse(mut reader: impl Read) -> Result<Self, GrfError> {
        let mut buf = BinaryReader::new(reader.borrow_mut()).unwrap();

        let mut header = Self::parse_header(buf).unwrap();

        let mut buf = BinaryReader::new(reader.borrow_mut()).unwrap();
        buf.skip(HEADER_SIZE);

        let mut entries = Self::parse_entries(buf).unwrap();

        let grf = GrfFile {
            header,
            entries: vec![],
        };
        
        Ok(grf)
    }

    fn parse_entries(mut buf: BinaryReader) -> Result<Vec<Entry>, GrfError> {
        Ok(vec![])
    }

    fn parse_header(mut buf: BinaryReader) -> Result<GrfHeader, GrfError> {
        let signature = buf.string(15);
        let _key = buf.string(15);
        let file_table_offset = buf.next_u32();
        let reserved_files = buf.next_u32();
        let entry_count = buf.next_u32() - (reserved_files + 7);
        let version = buf.next_u32();

        if signature != "Master of Magic" {
            panic!("Incorrect signature: {}", signature);
        }

        if version != 0x200 {
            panic!("Incorrect version: 0x{:2X}", version);
        }

        if signature != HEADER_SIGNATURE {
            return Err(GrfError::InvalidSignature);
        }

        if version != 0x200 {
            return Err(GrfError::InvalidVersion(version));
        }

        let mut header = GrfHeader {
            signature: signature.to_owned(),
            encryption_key: _key.to_owned(),
            file_table_offset,
            entry_count,
            reserved_files,
            version,
        };

        println!("{:#?}", header);

        Ok(header)
    }
}

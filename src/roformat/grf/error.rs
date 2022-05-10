use std::error::Error;
use std::fmt;

use crate::roformat::grf::error::GrfError::InvalidVersion;

#[derive(Debug)]
pub enum GrfError {
    InvalidSignature,
    InvalidVersion(u32),
}

impl fmt::Display for GrfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GrfError::InvalidSignature => write!(f, "invalid signature"),
            InvalidVersion(v) => write!(f, "invalid version 0x{:03X}", v.to_owned())
        }
    }
}

impl Error for GrfError {}

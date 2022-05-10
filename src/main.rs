#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#![deny(
/*missing_docs,*/
missing_debug_implementations,
missing_copy_implementations,
unstable_features,
unused_import_braces,
unused_qualifications,
)]

extern crate core;

use std::env;
use std::fmt::{Debug, DebugStruct, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::roformat::grf::GrfFile;

mod roformat;

fn main() {
    let file = File::open("./data/data.grf").unwrap();

    let grf = match GrfFile::parse(file) {
        Err(e) => panic!("error: {}", e),
        Ok(grf) => grf,
    };

    println!("{:#?}", grf)
}

fn read_file_bin<P: AsRef<Path>>(path: P) -> (File, Box<[u8]>) {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    (file, buffer.into_boxed_slice())
}

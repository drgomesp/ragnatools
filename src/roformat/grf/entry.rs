use bitflags::bitflags;

bitflags!(
    #[derive(Default)]
    #[repr(C)]
    pub struct Flags: u8 {
        const TYPE                = 0x01;
        const TYPE_ENCRYPT_MIXED  = 0x02;
        const TYPE_ENCRYPT_HEADER = 0x04;
    }
);

#[derive(Debug, Default)]
#[repr(C)]
// NOTE: @see https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
// We want this to behave just like in C in terms of order size and field alignment.
pub struct EntryHeader {
    compressed_size_aligned: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    flags: Flags,
}

#[derive(Debug, Default)]
pub struct Entry {
    pub(crate) name: String,
    pub(crate) header: EntryHeader,
}

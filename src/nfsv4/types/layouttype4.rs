#![allow(dead_code)]

use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.13: layouttype4
///
/// Identifies the layout type used in pNFS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LayoutType4 {
    /// LAYOUT4_NFSV4_1_FILES
    NfsV4_1Files = 0x1,

    /// LAYOUT4_OSD2_OBJECTS
    Osd2Objects = 0x2,

    /// LAYOUT4_BLOCK_VOLUME
    BlockVolume = 0x3,
}

impl TryFrom<i32> for LayoutType4 {
    type Error = Nfsv4Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Self::NfsV4_1Files),
            0x2 => Ok(Self::Osd2Objects),
            0x3 => Ok(Self::BlockVolume),
            _ => Err(Nfsv4Error::InvalidLayoutType(value)),
        }
    }
}

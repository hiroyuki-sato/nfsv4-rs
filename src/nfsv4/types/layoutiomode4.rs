#![allow(dead_code)]

use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.20: layoutiomode4
///
/// Specifies the I/O mode for a layout segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LayoutIOMode4 {
    /// LAYOUTIOMODE4_READ
    Read = 1,

    /// LAYOUTIOMODE4_RW
    ReadWrite = 2,

    /// LAYOUTIOMODE4_ANY
    Any = 3,
}

impl TryFrom<i32> for LayoutIOMode4 {
    type Error = Nfsv4Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Read),
            2 => Ok(Self::ReadWrite),
            3 => Ok(Self::Any),
            _ => Err(Nfsv4Error::InvalidLayoutIOMode(value)),
        }
    }
}

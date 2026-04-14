#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RestoreFh4Args;

impl RestoreFh4Args {
    pub fn decode(_r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {})
    }
    pub fn encode(&self, _w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        Ok(())
    }
}

/// RFC7531: RESTOREFH4res
///
/// Result of the RESTOREFH operation.
/// On success, CURRENT_FH becomes the saved filehandle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreFh4Res {
    /// NFS operation status.
    pub status: Stat4,
}

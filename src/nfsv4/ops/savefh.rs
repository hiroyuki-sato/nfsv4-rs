#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveFh4Args;

impl SaveFh4Args {
    pub fn decode(_r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {})
    }
    pub fn encode(&self, _w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        Ok(())
    }
}

/// RFC7531: SAVEFH4res
///
/// Result of the SAVEFH operation.
/// On success, SAVED_FH becomes the current filehandle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveFh4Res {
    /// NFS operation status.
    pub status: Stat4,
}

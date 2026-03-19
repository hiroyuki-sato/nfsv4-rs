#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: RESTOREFH4res
///
/// Result of the RESTOREFH operation.
/// On success, CURRENT_FH becomes the saved filehandle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreFh4Res {
    /// NFS operation status.
    pub status: Stat4,
}

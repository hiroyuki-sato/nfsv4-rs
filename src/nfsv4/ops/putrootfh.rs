#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: PUTROOTFH4res
///
/// Result of the PUTROOTFH operation.
/// On success, CURRENT_FH becomes the root filehandle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutRootFh4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

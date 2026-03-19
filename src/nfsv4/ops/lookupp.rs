#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: LOOKUPP4res
///
/// Result of the LOOKUPP operation (lookup parent directory).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupP4Res {
    /// NFS operation status.
    pub status: Stat4,
}

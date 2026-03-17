#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: PUTFH4args
///
/// Arguments for the PUTFH operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutFh4Args {
    /// Filehandle to set as CURRENT_FH.
    pub object: NfsFh4,
}

/// RFC7531: PUTFH4res
///
/// Result of the PUTFH operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutFh4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

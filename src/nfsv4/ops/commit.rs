#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: COMMIT4args
///
/// Arguments for the COMMIT operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit4Args {
    /// Starting offset of the range to commit.
    pub offset: Offset4,

    /// Number of bytes to commit.
    pub count: Count4,
}

/// RFC7531: COMMIT4resok
///
/// Successful result of the COMMIT operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit4ResOk {
    /// Write verifier returned by the server.
    pub writeverf: Verifier4,
}

/// RFC7531: COMMIT4res
///
/// Result of the COMMIT operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Commit4Res {
    /// Operation succeeded.
    Ok(Commit4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

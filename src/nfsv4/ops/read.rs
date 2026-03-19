#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: READ4args
///
/// Arguments for the READ operation.
/// CURRENT_FH must refer to a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Read4Args {
    /// State ID used for the read.
    pub stateid: StateId4,

    /// Starting offset in the file.
    pub offset: Offset4,

    /// Number of bytes to read.
    pub count: Count4,
}

/// RFC7531: READ4resok
///
/// Successful result of the READ operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Read4ResOk {
    /// Indicates end-of-file.
    pub eof: bool,

    /// Data returned by the server.
    pub data: Vec<u8>,
}

/// RFC7531: READ4res
///
/// Result of the READ operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Read4Res {
    /// Operation succeeded.
    Ok(Read4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

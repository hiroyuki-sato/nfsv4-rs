#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: stable_how4
///
/// Write stability level requested by the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StableHow4 {
    /// Data may be cached without stable storage.
    Unstable = 0,

    /// Data is committed to stable storage for file data only.
    DataSync = 1,

    /// Data and metadata are committed to stable storage.
    FileSync = 2,
}

/// RFC7531: WRITE4args
///
/// Arguments for the WRITE operation.
/// CURRENT_FH must refer to a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Write4Args {
    /// State ID used for the write.
    pub stateid: StateId4,

    /// Starting offset in the file.
    pub offset: Offset4,

    /// Requested write stability.
    pub stable: StableHow4,

    /// Data to be written.
    pub data: Vec<u8>,
}

/// RFC7531: WRITE4resok
///
/// Successful result of the WRITE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Write4ResOk {
    /// Number of bytes written.
    pub count: Count4,

    /// Stability level actually committed by the server.
    pub committed: StableHow4,

    /// Write verifier returned by the server.
    pub writeverf: Verifier4,
}

/// RFC7531: WRITE4res
///
/// Result of the WRITE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Write4Res {
    /// Operation succeeded.
    Ok(Write4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

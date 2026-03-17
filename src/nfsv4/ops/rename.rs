#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: RENAME4args
///
/// Arguments for the RENAME operation.
/// SAVED_FH must refer to the source directory.
/// CURRENT_FH must refer to the target directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rename4Args {
    /// Name of the source entry.
    pub oldname: Component4,

    /// New name in the target directory.
    pub newname: Component4,
}

/// RFC7531: RENAME4resok
///
/// Successful result of the RENAME operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rename4ResOk {
    /// Change information for the source directory.
    pub source_cinfo: ChangeInfo4,

    /// Change information for the target directory.
    pub target_cinfo: ChangeInfo4,
}

/// RFC7531: RENAME4res
///
/// Result of the RENAME operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rename4Res {
    /// Operation succeeded.
    Ok(Rename4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: GETATTR4args
///
/// Arguments for the GETATTR operation.
/// CURRENT_FH must refer to a file or directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAttr4Args {
    /// Bitmap of requested attributes.
    pub attr_request: Bitmap4,
}

/// RFC7531: GETATTR4resok
///
/// Successful result of the GETATTR operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAttr4ResOk {
    /// Returned object attributes.
    pub obj_attributes: Fattr4,
}

/// RFC7531: GETATTR4res
///
/// Result of the GETATTR operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetAttr4Res {
    /// Operation succeeded.
    Ok(GetAttr4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: ACCESS4args
///
/// Arguments for the ACCESS operation.
///
/// The client uses this operation to check access permissions
/// for the object identified by CURRENT_FH.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Access4Args {
    /// Bitmask of requested access types.
    /// Uses ACCESS4_* constants.
    pub access: u32,
}

/// RFC7531: ACCESS4resok
///
/// Successful result of the ACCESS operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Access4ResOk {
    /// Bitmask of access types supported by the server.
    pub supported: u32,

    /// Bitmask of access types granted to the client.
    pub access: u32,
}

/// RFC7531: ACCESS4res
///
/// Result of the ACCESS operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Access4Res {
    /// Operation succeeded.
    Ok(Access4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

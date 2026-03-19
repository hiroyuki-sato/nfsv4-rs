#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: GETFH4resok
///
/// Successful result of the GETFH operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFh4ResOk {
    /// Returned filehandle for CURRENT_FH.
    pub object: NfsFh4,
}

/// RFC7531: GETFH4res
///
/// Result of the GETFH operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetFh4Res {
    /// Operation succeeded.
    Ok(GetFh4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

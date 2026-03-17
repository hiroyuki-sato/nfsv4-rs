#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: REMOVE4args
///
/// Arguments for the REMOVE operation.
/// CURRENT_FH must refer to the containing directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remove4Args {
    /// Name of the directory entry to remove.
    pub target: Component4,
}

/// RFC7531: REMOVE4resok
///
/// Successful result of the REMOVE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remove4ResOk {
    /// Change information for the containing directory.
    pub cinfo: ChangeInfo4,
}

/// RFC7531: REMOVE4res
///
/// Result of the REMOVE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Remove4Res {
    /// Operation succeeded.
    Ok(Remove4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

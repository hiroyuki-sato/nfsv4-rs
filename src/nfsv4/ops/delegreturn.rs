#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: DELEGRETURN4args
///
/// Arguments for the DELEGRETURN operation.
/// CURRENT_FH must refer to the delegated file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegReturn4Args {
    /// Delegation state identifier.
    pub deleg_stateid: StateId4,
}

/// RFC7531: DELEGRETURN4res
///
/// Result of the DELEGRETURN operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegReturn4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

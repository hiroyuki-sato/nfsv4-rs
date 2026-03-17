#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: RELEASE_LOCKOWNER4args
///
/// Arguments for the RELEASE_LOCKOWNER operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseLockOwner4Args {
    /// Lock owner to be released.
    pub lock_owner: LockOwner4,
}

/// RFC7531: RELEASE_LOCKOWNER4res
///
/// Result of the RELEASE_LOCKOWNER operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseLockOwner4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

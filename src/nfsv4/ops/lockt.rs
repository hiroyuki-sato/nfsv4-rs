#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::Lock4Denied;
use crate::nfsv4::types::*;

/// RFC7531: LOCKT4args
///
/// Arguments for the LOCKT operation.
/// Used to test whether a lock could be granted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockT4Args {
    pub locktype: NfsLockType4,
    pub offset: Offset4,
    pub length: Length4,
    pub owner: LockOwner4,
}

/// RFC7531: LOCKT4res
///
/// Result of the LOCKT operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockT4Res {
    /// Lock test succeeded (no conflict).
    Ok,

    /// Lock request would be denied due to conflict.
    Denied(Lock4Denied),

    /// Other NFS error.
    Err(Stat4),
}

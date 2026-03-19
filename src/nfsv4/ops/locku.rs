#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: LOCKU4args
///
/// Arguments for the LOCKU operation.
/// Used to release a previously acquired lock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockU4Args {
    pub locktype: NfsLockType4,
    pub seqid: SeqId4,
    pub lock_stateid: StateId4,
    pub offset: Offset4,
    pub length: Length4,
}

/// RFC7531: LOCKU4res
///
/// Result of the LOCKU operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockU4Res {
    /// Operation succeeded.
    Ok(StateId4),

    /// Other NFS error.
    Err(Stat4),
}

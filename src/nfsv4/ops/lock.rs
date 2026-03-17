#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: open_to_lock_owner4
///
/// Used when transitioning from an open owner to a new lock owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenToLockOwner4 {
    /// Sequence ID for the open owner.
    pub open_seqid: SeqId4,

    /// State ID returned by the OPEN operation.
    pub open_stateid: StateId4,

    /// Sequence ID for the new lock owner.
    pub lock_seqid: SeqId4,

    /// Lock owner information.
    pub lock_owner: LockOwner4,
}

/// RFC7531: exist_lock_owner4
///
/// Used when an existing lock owner continues to request locks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExistLockOwner4 {
    /// State ID for the existing lock owner.
    pub lock_stateid: StateId4,

    /// Sequence ID for the lock owner.
    pub lock_seqid: SeqId4,
}

/// RFC7531: locker4
///
/// Indicates whether a new lock owner is being created
/// or an existing lock owner is used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Locker4 {
    /// A new lock owner is created from an open owner.
    New(OpenToLockOwner4),

    /// An existing lock owner continues locking.
    Existing(ExistLockOwner4),
}

/// RFC7531: LOCK4args
///
/// Arguments for the LOCK operation.
/// CURRENT_FH must refer to the file being locked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lock4Args {
    /// Type of lock requested.
    pub locktype: NfsLockType4,

    /// Indicates whether the lock is being reclaimed.
    pub reclaim: bool,

    /// Starting offset of the lock range.
    pub offset: Offset4,

    /// Length of the lock range.
    pub length: Length4,

    /// Lock owner information.
    pub locker: Locker4,
}

/// RFC7531: LOCK4denied
///
/// Information returned when a lock request is denied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lock4Denied {
    /// Starting offset of the conflicting lock.
    pub offset: Offset4,

    /// Length of the conflicting lock.
    pub length: Length4,

    /// Type of the conflicting lock.
    pub locktype: NfsLockType4,

    /// Owner of the conflicting lock.
    pub owner: LockOwner4,
}

/// RFC7531: LOCK4resok
///
/// Successful result of the LOCK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lock4ResOk {
    /// State ID representing the acquired lock.
    pub lock_stateid: StateId4,
}

/// RFC7531: LOCK4res
///
/// Result of the LOCK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lock4Res {
    /// Operation succeeded.
    Ok(Lock4ResOk),

    /// Lock request denied due to conflict.
    Denied(Lock4Denied),

    /// Other NFS error.
    Err(NfsStat4),
}

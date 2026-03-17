#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

pub mod access;
pub mod close;
pub mod commit;
pub mod create;
pub mod delegpurge;
pub mod delegreturn;
pub mod getattr;
pub mod getfh;
pub mod link;
pub mod lock;
pub mod lockt;
pub mod locku;
pub mod lookup;
pub mod nverify;
pub mod open;
pub mod putfh;
pub mod putpubfh;
pub mod putrootfh;
pub mod read;
pub mod readdir;
pub mod readlink;
pub mod remove;
pub mod rename;
pub mod renew;
pub mod restore;
pub mod savefh;
pub mod secinfo;
pub mod setattr;

pub use access::*;
pub use close::*;
pub use commit::*;
pub use create::*;
pub use delegpurge::*;
pub use delegreturn::*;
pub use getattr::*;
pub use getfh::*;
pub use link::*;
pub use lock::*;
pub use lockt::*;
pub use locku::*;
pub use lookup::*;
pub use nverify::*;
pub use open::*;
pub use putfh::*;
pub use putpubfh::*;
pub use putrootfh::*;
pub use read::*;
pub use readdir::*;
pub use readlink::*;
pub use remove::*;
pub use rename::*;
pub use renew::*;
pub use restore::*;
pub use savefh::*;
pub use secinfo::*;
pub use setattr::*;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: SETCLIENTID4args
///
/// Arguments for the SETCLIENTID operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientId4Args {
    /// Client identification information.
    pub client: NfsClientId4,

    /// Callback program information.
    pub callback: CbClient4,

    /// Client-chosen callback identifier.
    pub callback_ident: u32,
}

/// RFC7531: SETCLIENTID4resok
///
/// Successful result of the SETCLIENTID operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientId4ResOk {
    /// Assigned client identifier.
    pub clientid: ClientId4,

    /// Verifier used in SETCLIENTID_CONFIRM.
    pub setclientid_confirm: Verifier4,
}

/// RFC7531: SETCLIENTID4res
///
/// Result of the SETCLIENTID operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetClientId4Res {
    /// Operation succeeded.
    Ok(SetClientId4ResOk),

    /// Client ID is already in use by another client address.
    ClientIdInUse(ClientAddr4),

    /// Operation failed with another NFS error status.
    Err(NfsStat4),
}

/// RFC7531: SETCLIENTID_CONFIRM4args
///
/// Arguments for the SETCLIENTID_CONFIRM operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientIdConfirm4Args {
    /// Client identifier returned by SETCLIENTID.
    pub clientid: ClientId4,

    /// Confirmation verifier returned by SETCLIENTID.
    pub setclientid_confirm: Verifier4,
}

/// RFC7531: SETCLIENTID_CONFIRM4res
///
/// Result of the SETCLIENTID_CONFIRM operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientIdConfirm4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

/// RFC7531: VERIFY4args
///
/// Arguments for the VERIFY operation.
/// CURRENT_FH must refer to the target object.
///
/// The operation succeeds only if the specified attributes
/// match the object's attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verify4Args {
    /// Attributes to compare against the object.
    pub obj_attributes: Fattr4,
}

/// RFC7531: VERIFY4res
///
/// Result of the VERIFY operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verify4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

/// RFC7531: stable_how4
///
/// Write stability level requested by the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StableHow4 {
    /// Data may be cached without stable storage.
    Unstable = 0,

    /// Data is committed to stable storage for file data only.
    DataSync = 1,

    /// Data and metadata are committed to stable storage.
    FileSync = 2,
}

/// RFC7531: WRITE4args
///
/// Arguments for the WRITE operation.
/// CURRENT_FH must refer to a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Write4Args {
    /// State ID used for the write.
    pub stateid: StateId4,

    /// Starting offset in the file.
    pub offset: Offset4,

    /// Requested write stability.
    pub stable: StableHow4,

    /// Data to be written.
    pub data: Vec<u8>,
}

/// RFC7531: WRITE4resok
///
/// Successful result of the WRITE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Write4ResOk {
    /// Number of bytes written.
    pub count: Count4,

    /// Stability level actually committed by the server.
    pub committed: StableHow4,

    /// Write verifier returned by the server.
    pub writeverf: Verifier4,
}

/// RFC7531: WRITE4res
///
/// Result of the WRITE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Write4Res {
    /// Operation succeeded.
    Ok(Write4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

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

/// RFC7531: ILLEGAL4res
///
/// Result returned when the server encounters an undefined
/// or unsupported operation in a COMPOUND request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Illegal4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

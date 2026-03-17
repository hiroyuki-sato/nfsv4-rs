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
pub mod readdir;

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
pub use readdir::*;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: READ4args
///
/// Arguments for the READ operation.
/// CURRENT_FH must refer to a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Read4Args {
    /// State ID used for the read.
    pub stateid: StateId4,

    /// Starting offset in the file.
    pub offset: Offset4,

    /// Number of bytes to read.
    pub count: Count4,
}

/// RFC7531: READ4resok
///
/// Successful result of the READ operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Read4ResOk {
    /// Indicates end-of-file.
    pub eof: bool,

    /// Data returned by the server.
    pub data: Vec<u8>,
}

/// RFC7531: READ4res
///
/// Result of the READ operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Read4Res {
    /// Operation succeeded.
    Ok(Read4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

/// RFC7531: READLINK4resok
///
/// Successful result of the READLINK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadLink4ResOk {
    /// Symbolic link target.
    pub link: LinkText4,
}

/// RFC7531: READLINK4res
///
/// Result of the READLINK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadLink4Res {
    /// Operation succeeded.
    Ok(ReadLink4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

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

/// RFC7531: RENAME4args
///
/// Arguments for the RENAME operation.
/// SAVED_FH must refer to the source directory.
/// CURRENT_FH must refer to the target directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rename4Args {
    /// Name of the source entry.
    pub oldname: Component4,

    /// New name in the target directory.
    pub newname: Component4,
}

/// RFC7531: RENAME4resok
///
/// Successful result of the RENAME operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rename4ResOk {
    /// Change information for the source directory.
    pub source_cinfo: ChangeInfo4,

    /// Change information for the target directory.
    pub target_cinfo: ChangeInfo4,
}

/// RFC7531: RENAME4res
///
/// Result of the RENAME operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rename4Res {
    /// Operation succeeded.
    Ok(Rename4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

/// RFC7531: RENEW4args
///
/// Arguments for the RENEW operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renew4Args {
    /// Client identifier to renew.
    pub clientid: ClientId4,
}

/// RFC7531: RENEW4res
///
/// Result of the RENEW operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renew4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

/// RFC7531: RESTOREFH4res
///
/// Result of the RESTOREFH operation.
/// On success, CURRENT_FH becomes the saved filehandle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreFh4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

/// RFC7531: SAVEFH4res
///
/// Result of the SAVEFH operation.
/// On success, SAVED_FH becomes the current filehandle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveFh4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

/// RFC7531: SECINFO4args
///
/// Arguments for the SECINFO operation.
/// CURRENT_FH must refer to a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecInfo4Args {
    /// Name of the object whose security information is requested.
    pub name: Component4,
}

/// RFC2203: rpc_gss_svc_t
///
/// RPCSEC_GSS service type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RpcGssSvcT {
    /// No security service.
    None = 1,

    /// Integrity protection service.
    Integrity = 2,

    /// Privacy protection service.
    Privacy = 3,
}

/// RFC7531: rpcsec_gss_info
///
/// RPCSEC_GSS security information returned by SECINFO.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcSecGssInfo {
    /// Security mechanism OID.
    pub oid: SecOid4,

    /// Quality of protection.
    pub qop: Qop4,

    /// RPCSEC_GSS service type.
    pub service: RpcGssSvcT,
}

/// RPCSEC_GSS flavor value defined by RFC2203.
pub const RPCSEC_GSS: u32 = 6;

/// RFC7531: secinfo4
///
/// Security flavor information returned by SECINFO.
///
/// The discriminator is the RPC authentication flavor number.
/// For flavors other than RPCSEC_GSS, no additional data is returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecInfo4 {
    /// RPCSEC_GSS flavor with additional GSS information.
    RpcSecGss(RpcSecGssInfo),

    /// Other security flavor with no additional flavor-specific data.
    Other(u32),
}

/// RFC7531: SECINFO4resok
///
/// Successful result payload of the SECINFO operation.
///
/// In XDR this is:
/// `typedef secinfo4 SECINFO4resok<>;`
pub type SecInfo4ResOk = Vec<SecInfo4>;

/// RFC7531: SECINFO4res
///
/// Result of the SECINFO operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecInfo4Res {
    /// Operation succeeded.
    Ok(SecInfo4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

/// RFC7531: SETATTR4args
///
/// Arguments for the SETATTR operation.
/// CURRENT_FH must refer to the target object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetAttr4Args {
    /// State ID used for attribute changes.
    pub stateid: StateId4,

    /// Attributes to be set on the object.
    pub obj_attributes: Fattr4,
}

/// RFC7531: SETATTR4res
///
/// Result of the SETATTR operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetAttr4Res {
    /// NFS operation status.
    pub status: NfsStat4,

    /// Bitmap of attributes successfully set.
    pub attrsset: Bitmap4,
}

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

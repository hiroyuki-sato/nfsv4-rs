#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

pub mod access;
pub mod close;
pub mod commit;
pub mod create;
pub mod getattr;
pub mod lookup;
pub mod putrootfh;
pub mod readdir;

pub use access::*;
pub use close::*;
pub use commit::*;
pub use create::*;
pub use getattr::*;
pub use lookup::*;
pub use putrootfh::*;
pub use readdir::*;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: DELEGPURGE4args
///
/// Arguments for the DELEGPURGE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegPurge4Args {
    /// Client identifier whose delegations should be purged.
    pub clientid: ClientId4,
}

/// RFC7531: DELEGPURGE4res
///
/// Result of the DELEGPURGE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegPurge4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

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
    Err(NfsStat4),
}

/// RFC7531: LINK4args
///
/// Arguments for the LINK operation.
/// SAVED_FH must refer to the source object.
/// CURRENT_FH must refer to the target directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link4Args {
    /// New name of the linked object in the target directory.
    pub newname: Component4,
}

/// RFC7531: LINK4resok
///
/// Successful result of the LINK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link4ResOk {
    /// Change information for the target directory.
    pub cinfo: ChangeInfo4,
}

/// RFC7531: LINK4res
///
/// Result of the LINK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Link4Res {
    /// Operation succeeded.
    Ok(Link4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

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
    Err(NfsStat4),
}

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
    Err(NfsStat4),
}

/// RFC7531: NVERIFY4args
///
/// Arguments for the NVERIFY operation.
/// CURRENT_FH must refer to the target object.
///
/// The operation succeeds only if the specified attributes
/// do NOT match the object's attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NVerify4Args {
    /// Attributes to compare against the object.
    pub obj_attributes: Fattr4,
}

/// RFC7531: NVERIFY4res
///
/// Result of the NVERIFY operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NVerify4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

/// Share access flags used in the OPEN operation.
pub const OPEN4_SHARE_ACCESS_READ: u32 = 0x00000001;
pub const OPEN4_SHARE_ACCESS_WRITE: u32 = 0x00000002;
pub const OPEN4_SHARE_ACCESS_BOTH: u32 = 0x00000003;

/// Share deny flags used in the OPEN operation.
pub const OPEN4_SHARE_DENY_NONE: u32 = 0x00000000;
pub const OPEN4_SHARE_DENY_READ: u32 = 0x00000001;
pub const OPEN4_SHARE_DENY_WRITE: u32 = 0x00000002;
pub const OPEN4_SHARE_DENY_BOTH: u32 = 0x00000003;

/// RFC7531: createmode4
///
/// Specifies how file creation should behave when OPEN is used
/// with the CREATE flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CreateMode4 {
    Unchecked = 0,
    Guarded = 1,
    Exclusive = 2,
}

/// RFC7531: createhow4
///
/// Additional data required when creating a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateHow4 {
    /// UNCHECKED4 or GUARDED4
    ///
    /// Attributes used when creating the object.
    CreateAttrs(Fattr4),

    /// EXCLUSIVE4
    ///
    /// Verifier used to detect duplicate requests.
    Exclusive(Verifier4),
}

/// RFC7531: opentype4
///
/// Indicates whether the OPEN operation creates a new file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum OpenType4 {
    NoCreate = 0,
    Create = 1,
}

/// RFC7531: openflag4
///
/// Flags controlling file creation behavior in OPEN.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenFlag4 {
    /// File is opened without creation.
    NoCreate,

    /// File is created according to the specified method.
    Create(CreateHow4),
}

/// RFC7531: limit_by4
///
/// Specifies how a space limit is described.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LimitBy4 {
    /// Limit specified as file size in bytes.
    Size = 1,

    /// Limit specified as number of blocks.
    Blocks = 2,
}

/// RFC7531: nfs_modified_limit4
///
/// Space limit expressed as block count and block size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NfsModifiedLimit4 {
    /// Number of blocks.
    pub num_blocks: u32,

    /// Number of bytes per block.
    pub bytes_per_block: u32,
}

/// RFC7531: nfs_space_limit4
///
/// Space limit used for OPEN delegation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NfsSpaceLimit4 {
    /// Limit specified as file size in bytes.
    Size(u64),

    /// Limit specified by block count and block size.
    Blocks(NfsModifiedLimit4),
}

/// RFC7531: open_delegation_type4
///
/// Type of OPEN delegation granted by the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum OpenDelegationType4 {
    /// No delegation granted.
    None = 0,

    /// Read delegation granted.
    Read = 1,

    /// Write delegation granted.
    Write = 2,
}

/// RFC7531: open_claim_type4
///
/// Type of claim used in the OPEN operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum OpenClaimType4 {
    /// Normal open by filename.
    Null = 0,

    /// Reclaim after server restart.
    Previous = 1,

    /// Open based on current delegation.
    DelegateCurrent = 2,

    /// Reclaim previous delegation.
    DelegatePrevious = 3,
}

/// RFC7531: open_claim_delegate_cur4
///
/// Claim data for CLAIM_DELEGATE_CUR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenClaimDelegateCurrent4 {
    /// Delegation state ID.
    pub delegate_stateid: StateId4,

    /// File name being claimed.
    pub file: Component4,
}

/// RFC7531: open_claim4
///
/// Describes how the client claims the right to open a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenClaim4 {
    /// CLAIM_NULL
    ///
    /// Ordinary OPEN of the specified file.
    /// CURRENT_FH must refer to the parent directory.
    Null(Component4),

    /// CLAIM_PREVIOUS
    ///
    /// Reclaim an open that existed before a server reboot.
    /// CURRENT_FH must refer to the file being reclaimed.
    Previous(OpenDelegationType4),

    /// CLAIM_DELEGATE_CUR
    ///
    /// Claim based on a delegation currently held by the client.
    /// CURRENT_FH must refer to the parent directory.
    DelegateCurrent(OpenClaimDelegateCurrent4),

    /// CLAIM_DELEGATE_PREV
    ///
    /// Claim based on a delegation from a previous client instance.
    /// CURRENT_FH must refer to the parent directory.
    DelegatePrevious(Component4),
}

/// RFC7531: OPEN4args
///
/// Arguments for the OPEN operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Open4Args {
    /// Sequence ID for the open owner.
    pub seqid: SeqId4,

    /// Share access flags (OPEN4_SHARE_ACCESS_*).
    pub share_access: u32,

    /// Share deny flags (OPEN4_SHARE_DENY_*).
    pub share_deny: u32,

    /// Open owner identifier.
    pub owner: OpenOwner4,

    /// File creation/open flags.
    pub openhow: OpenFlag4,

    /// Claim describing how the client identifies the file.
    pub claim: OpenClaim4,
}

/// RFC7531: open_read_delegation4
///
/// Delegation information returned for a read delegation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenReadDelegation4 {
    /// State ID representing the delegation.
    pub stateid: StateId4,

    /// Indicates whether the delegation has been pre-recalled.
    pub recall: bool,

    /// Permissions allowing users to open the file without an ACCESS call.
    pub permissions: NfsAce4,
}

/// RFC7531: open_write_delegation4
///
/// Delegation information returned for a write delegation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenWriteDelegation4 {
    /// State ID representing the delegation.
    pub stateid: StateId4,

    /// Indicates whether the delegation has been pre-recalled.
    pub recall: bool,

    /// Space limit used to determine when data must be flushed to the server.
    pub space_limit: NfsSpaceLimit4,

    /// Permissions allowing users to open the file without an ACCESS call.
    pub permissions: NfsAce4,
}

/// RFC7531: open_delegation4
///
/// Delegation information returned by the OPEN operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenDelegation4 {
    /// No delegation granted.
    None,

    /// Read delegation granted.
    Read(OpenReadDelegation4),

    /// Write delegation granted.
    Write(OpenWriteDelegation4),
}

/// Result flag indicating that the client must confirm the open.
pub const OPEN4_RESULT_CONFIRM: u32 = 0x00000002;

/// Result flag indicating POSIX-style file locking behavior.
pub const OPEN4_RESULT_LOCKTYPE_POSIX: u32 = 0x00000004;

/// RFC7531: OPEN4resok
///
/// Successful result of the OPEN operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Open4ResOk {
    /// State ID for the open.
    pub stateid: StateId4,

    /// Change information for the containing directory.
    pub cinfo: ChangeInfo4,

    /// Result flags (OPEN4_RESULT_*).
    pub rflags: u32,

    /// Bitmap indicating which attributes were set during create.
    pub attrset: Bitmap4,

    /// Delegation information returned by the server.
    pub delegation: OpenDelegation4,
}

/// RFC7531: OPEN4res
///
/// Result of the OPEN operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Open4Res {
    /// Operation succeeded.
    Ok(Open4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

/// RFC7531: OPENATTR4args
///
/// Arguments for the OPENATTR operation.
/// CURRENT_FH must refer to the target object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenAttr4Args {
    /// If true, create the named attribute directory if it does not exist.
    pub createdir: bool,
}

/// RFC7531: OPENATTR4res
///
/// Result of the OPENATTR operation.
/// CURRENT_FH becomes the named attribute directory on success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenAttr4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

/// RFC7531: OPEN_CONFIRM4args
///
/// Arguments for the OPEN_CONFIRM operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenConfirm4Args {
    /// State ID returned by OPEN.
    pub open_stateid: StateId4,

    /// Sequence ID for the open owner.
    pub seqid: SeqId4,
}

/// RFC7531: OPEN_CONFIRM4resok
///
/// Successful result of the OPEN_CONFIRM operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenConfirm4ResOk {
    /// Updated state ID for the open.
    pub open_stateid: StateId4,
}

/// RFC7531: OPEN_CONFIRM4res
///
/// Result of the OPEN_CONFIRM operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenConfirm4Res {
    /// Operation succeeded.
    Ok(OpenConfirm4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

/// RFC7531: OPEN_DOWNGRADE4args
///
/// Arguments for the OPEN_DOWNGRADE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenDowngrade4Args {
    /// State ID for the open.
    pub open_stateid: StateId4,

    /// Sequence ID for the open owner.
    pub seqid: SeqId4,

    /// New share access flags (OPEN4_SHARE_ACCESS_*).
    pub share_access: u32,

    /// New share deny flags (OPEN4_SHARE_DENY_*).
    pub share_deny: u32,
}

/// RFC7531: OPEN_DOWNGRADE4resok
///
/// Successful result of the OPEN_DOWNGRADE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenDowngrade4ResOk {
    /// Updated state ID for the open.
    pub open_stateid: StateId4,
}

/// RFC7531: OPEN_DOWNGRADE4res
///
/// Result of the OPEN_DOWNGRADE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenDowngrade4Res {
    /// Operation succeeded.
    Ok(OpenDowngrade4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

/// RFC7531: PUTFH4args
///
/// Arguments for the PUTFH operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutFh4Args {
    /// Filehandle to set as CURRENT_FH.
    pub object: NfsFh4,
}

/// RFC7531: PUTFH4res
///
/// Result of the PUTFH operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutFh4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

/// RFC7531: PUTPUBFH4res
///
/// Result of the PUTPUBFH operation.
/// On success, CURRENT_FH becomes the public filehandle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutPubFh4Res {
    /// NFS operation status.
    pub status: NfsStat4,
}

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

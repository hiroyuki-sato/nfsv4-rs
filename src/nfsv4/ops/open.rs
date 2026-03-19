#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

// TODO Separate this file into multiple files as more operations are implemented.
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
    Err(Stat4),
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
    pub status: Stat4,
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
    Err(Stat4),
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
    Err(Stat4),
}

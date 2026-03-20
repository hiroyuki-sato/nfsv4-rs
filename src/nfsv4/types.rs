#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

pub mod bitmap4;
pub mod change_policy4;
pub mod client_owner4;
pub mod device_addr4;
pub mod fattr4;
pub mod fs_location4;
pub mod fs_locations4;
pub mod fsid4;
pub mod ftype4;
pub mod layout4;
pub mod layout_content4;
pub mod layouthint4;
pub mod layoutiomode4;
pub mod layouttype4;
pub mod layoutupdate4;
pub mod mdsthreshold4;
pub mod mode4;
pub mod netaddr4;
pub mod nfs_impl_id4;
pub mod nfstime4;
pub mod secoid4;
pub mod server_owner4;
pub mod settime4;
pub mod stat4;
pub mod state_owner4;
pub mod threshold_item4;
pub mod time_how4;

pub use crate::nfsv4::types::bitmap4::*;
pub use crate::nfsv4::types::change_policy4::*;
pub use crate::nfsv4::types::client_owner4::*;
pub use crate::nfsv4::types::device_addr4::*;
pub use crate::nfsv4::types::fattr4::*;
pub use crate::nfsv4::types::fs_location4::*;
pub use crate::nfsv4::types::fs_locations4::*;
pub use crate::nfsv4::types::fsid4::*;
pub use crate::nfsv4::types::ftype4::*;
pub use crate::nfsv4::types::layout_content4::*;
pub use crate::nfsv4::types::layout4::*;
pub use crate::nfsv4::types::layouthint4::*;
pub use crate::nfsv4::types::layoutiomode4::*;
pub use crate::nfsv4::types::layouttype4::*;
pub use crate::nfsv4::types::layoutupdate4::*;
pub use crate::nfsv4::types::mdsthreshold4::*;
pub use crate::nfsv4::types::mode4::*;
pub use crate::nfsv4::types::netaddr4::*;
pub use crate::nfsv4::types::nfs_impl_id4::*;
pub use crate::nfsv4::types::nfstime4::*;
pub use crate::nfsv4::types::secoid4::*;
pub use crate::nfsv4::types::server_owner4::*;
pub use crate::nfsv4::types::settime4::*;
pub use crate::nfsv4::types::stat4::*;
pub use crate::nfsv4::types::state_owner4::*;
pub use crate::nfsv4::types::threshold_item4::*;
pub use crate::nfsv4::types::time_how4::*;

pub const FHSIZE: usize = 128;
pub const VERIFIER_SIZE: usize = 8;
pub const OTHER_SIZE: usize = 12;
pub const OPAQUE_LIMIT: usize = 1024;

use crate::error::Nfsv4Error;

// RFC7531 Basic data types (selected)

pub type Attrlist4 = Vec<u8>;

pub type ChangeId4 = u64;
pub type ClientId4 = u64;
pub type Count4 = u32;
pub type Length4 = u64;

pub type NfsCookie4 = u64;
pub type NfsLease4 = u32;

pub type Offset4 = u64;
pub type Qop4 = u32;
pub type SeqId4 = u32;

pub type NfsLockId4 = u64;

pub type LinkText4 = Vec<u8>;

pub type Component4 = String;
pub type Pathname4 = Vec<Component4>;

/// opaque nfs_fh4<NFS4_FHSIZE>
pub type NfsFh4 = Vec<u8>;

/// opaque verifier4[NFS4_VERIFIER_SIZE]
pub type Verifier4 = [u8; VERIFIER_SIZE];

pub const ACL4_SUPPORT_ALLOW_ACL: u32 = 0x00000001;
pub const ACL4_SUPPORT_DENY_ACL: u32 = 0x00000002;
pub const ACL4_SUPPORT_AUDIT_ACL: u32 = 0x00000004;
pub const ACL4_SUPPORT_ALARM_ACL: u32 = 0x00000008;

pub type AclSupport4 = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AceType4 {
    AccessAllowed = 0x00000000,
    AccessDenied = 0x00000001,
    SystemAudit = 0x00000002,
    SystemAlarm = 0x00000003,
}

pub type AceTypeRaw = u32;

/// ACE Flag
pub type AceFlag4 = u32;

pub const ACE4_FILE_INHERIT_ACE: AceFlag4 = 0x00000001;
pub const ACE4_DIRECTORY_INHERIT_ACE: AceFlag4 = 0x00000002;
pub const ACE4_NO_PROPAGATE_INHERIT_ACE: AceFlag4 = 0x00000004;
pub const ACE4_INHERIT_ONLY_ACE: AceFlag4 = 0x00000008;
pub const ACE4_SUCCESSFUL_ACCESS_ACE_FLAG: AceFlag4 = 0x00000010;
pub const ACE4_FAILED_ACCESS_ACE_FLAG: AceFlag4 = 0x00000020;
pub const ACE4_IDENTIFIER_GROUP: AceFlag4 = 0x00000040;

/// ACE Mask

pub type AceMask4 = u32;

pub const ACE4_READ_DATA: AceMask4 = 0x00000001;
pub const ACE4_LIST_DIRECTORY: AceMask4 = 0x00000001;

pub const ACE4_WRITE_DATA: AceMask4 = 0x00000002;
pub const ACE4_ADD_FILE: AceMask4 = 0x00000002;

pub const ACE4_APPEND_DATA: AceMask4 = 0x00000004;
pub const ACE4_ADD_SUBDIRECTORY: AceMask4 = 0x00000004;

pub const ACE4_READ_NAMED_ATTRS: AceMask4 = 0x00000008;
pub const ACE4_WRITE_NAMED_ATTRS: AceMask4 = 0x00000010;

pub const ACE4_EXECUTE: AceMask4 = 0x00000020;
pub const ACE4_DELETE_CHILD: AceMask4 = 0x00000040;

pub const ACE4_READ_ATTRIBUTES: AceMask4 = 0x00000080;
pub const ACE4_WRITE_ATTRIBUTES: AceMask4 = 0x00000100;

pub const ACE4_DELETE: AceMask4 = 0x00010000;
pub const ACE4_READ_ACL: AceMask4 = 0x00020000;
pub const ACE4_WRITE_ACL: AceMask4 = 0x00040000;
pub const ACE4_WRITE_OWNER: AceMask4 = 0x00080000;
pub const ACE4_SYNCHRONIZE: AceMask4 = 0x00100000;

/// ACE4_GENERIC_READ =
pub const ACE4_GENERIC_READ: AceMask4 =
    ACE4_READ_ACL | ACE4_READ_DATA | ACE4_READ_ATTRIBUTES | ACE4_SYNCHRONIZE;

/// ACE4_GENERIC_WRITE =
pub const ACE4_GENERIC_WRITE: AceMask4 = ACE4_READ_ACL
    | ACE4_WRITE_DATA
    | ACE4_WRITE_ATTRIBUTES
    | ACE4_WRITE_ACL
    | ACE4_APPEND_DATA
    | ACE4_SYNCHRONIZE;

/// ACE4_GENERIC_EXECUTE =
pub const ACE4_GENERIC_EXECUTE: AceMask4 =
    ACE4_READ_ACL | ACE4_READ_ATTRIBUTES | ACE4_EXECUTE | ACE4_SYNCHRONIZE;

/// RFC7531: nfsace4
///
/// Access Control Entry definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NfsAce4 {
    /// acetype4
    ///
    /// ACE type (allow, deny, audit, alarm)
    pub ace_type: AceType4,

    /// aceflag4
    ///
    /// ACE flag bitmask
    pub flag: AceFlag4,

    /// acemask4
    ///
    /// Access permission mask
    pub access_mask: AceMask4,

    /// utf8str_mixed
    ///
    /// Principal or group name the ACE applies to
    pub who: String,
}

/// RFC7531: specdata4
///
/// Special data/attribute associated with
/// file types NF4BLK and NF4CHR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpecData4 {
    /// major device number
    pub major: u32,

    /// minor device number
    pub minor: u32,
}

/// RFC7531: change_info4
///
/// Information about a change made to a file or directory.
/// Used by operations such as CREATE, REMOVE, LINK, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeInfo4 {
    /// Indicates whether the change was atomic.
    pub atomic: bool,

    /// Change attribute value before the operation.
    pub before: ChangeId4,

    /// Change attribute value after the operation.
    pub after: ChangeId4,
}

/// RFC7531: clientaddr4
///
/// Client network address used for callbacks.
///
/// See also RFC1833 (rpcbind).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientAddr4 {
    /// Network identifier (e.g., "tcp").
    pub r_netid: String,

    /// Universal address string.
    pub r_addr: String,
}

/// RFC7531: cb_client4
///
/// Callback program information provided by the client.
/// Used by the server to perform callback RPCs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CbClient4 {
    /// RPC program number for callbacks.
    pub cb_program: u32,

    /// Client network location for callbacks.
    pub cb_location: ClientAddr4,
}

/// RFC7531: stateid4
///
/// State identifier used for OPEN, LOCK, READ, WRITE operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateId4 {
    /// Sequence identifier for the state.
    pub seqid: u32,

    /// Opaque state data provided by the server.
    pub other: [u8; OTHER_SIZE],
}

/// RFC7531: nfs_client_id4
///
/// Client identifier used during client registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NfsClientId4 {
    /// Verifier used to detect client reboot.
    pub verifier: Verifier4,

    /// Opaque client identifier.
    pub id: Vec<u8>,
}

/// RFC7531: open_owner4
///
/// Identifies an OPEN owner (client + owner identifier).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenOwner4 {
    pub clientid: ClientId4,
    pub owner: Vec<u8>,
}

/// RFC7531: lock_owner4
///
/// Identifies a LOCK owner (client + owner identifier).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockOwner4 {
    pub clientid: ClientId4,
    pub owner: Vec<u8>,
}

/// RFC7531: nfs_lock_type4
///
/// Lock types used in NFSv4 locking operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NfsLockType4 {
    /// Shared (read) lock.
    Read = 1,

    /// Exclusive (write) lock.
    Write = 2,

    /// Blocking shared (read) lock.
    ReadBlocking = 3,

    /// Blocking exclusive (write) lock.
    WriteBlocking = 4,
}

/// ACCESS operation bitmask values.
///
/// These flags indicate the access permissions that the client
/// wishes to check with the ACCESS operation.
pub const ACCESS4_READ: u32 = 0x00000001;
pub const ACCESS4_LOOKUP: u32 = 0x00000002;
pub const ACCESS4_MODIFY: u32 = 0x00000004;
pub const ACCESS4_EXTEND: u32 = 0x00000008;
pub const ACCESS4_DELETE: u32 = 0x00000010;
pub const ACCESS4_EXECUTE: u32 = 0x00000020;

/// RFC8881: NFS4_SESSIONID_SIZE
///
/// Fixed size of a session identifier in NFSv4.1.
pub const SESSIONID_SIZE: usize = 16;

/// RFC8881: sequenceid4
///
/// Sequence number used for session-related operations
/// such as EXCHANGE_ID, CREATE_SESSION, SEQUENCE, and CB_SEQUENCE.
pub type SequenceId4 = u32;

/// RFC8881: sessionid4
///
/// Fixed-size session identifier used in NFSv4.1 sessions.
pub type SessionId4 = [u8; SESSIONID_SIZE];

/// RFC8881: slotid4
///
/// Slot identifier used for session-related operations
/// such as SEQUENCE and CB_SEQUENCE.
pub type SlotId4 = u32;

/// RFC8881 Section 3.3.14: deviceid4
///
/// Fixed-size identifier for a device (16 bytes).
pub const NFS4_DEVICEID4_SIZE: usize = 16;

/// opaque deviceid4[NFS4_DEVICEID4_SIZE]
pub type DeviceId4 = [u8; NFS4_DEVICEID4_SIZE];

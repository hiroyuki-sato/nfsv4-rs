#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

pub mod bitmap4;
pub mod ftype4;
pub mod mode4;
pub mod nfsstat4;

pub use crate::nfsv4::types::bitmap4::*;
pub use crate::nfsv4::types::ftype4::*;
pub use crate::nfsv4::types::mode4::*;
pub use crate::nfsv4::types::nfsstat4::*;

pub const FHSIZE: usize = 128;
pub const VERIFIER_SIZE: usize = 8;
pub const OTHER_SIZE: usize = 12;
pub const OPAQUE_LIMIT: usize = 1024;

use crate::error::Nfsv4Error;

// RFC7531 Basic data types (selected)

pub type Attrlist4 = Vec<u8>;
pub type Bitmap4 = Vec<u32>;

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

pub type SecOid4 = Vec<u8>;
pub type LinkText4 = Vec<u8>;

pub type Component4 = String;
pub type Pathname4 = Vec<Component4>;

/// opaque nfs_fh4<NFS4_FHSIZE>
pub type NfsFh4 = Vec<u8>;

/// opaque verifier4[NFS4_VERIFIER_SIZE]
pub type Verifier4 = [u8; VERIFIER_SIZE];

/// RFC7531: nfstime4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NfsTime4 {
    pub seconds: i64,
    pub nseconds: u32,
}

/// RFC7531: time_how4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TimeHow4 {
    SetToServerTime = 0,
    SetToClientTime = 1,
}

/// RFC7531: settime4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetTime4 {
    SetToServerTime,
    SetToClientTime(NfsTime4),
}

/// RFC7531: fsid4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fsid4 {
    pub major: u64,
    pub minor: u64,
}

/// RFC7531: fs_location4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsLocation4 {
    pub server: Vec<String>,
    pub rootpath: Pathname4,
}

/// RFC7531: fs_locations4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsLocations4 {
    pub fs_root: Pathname4,
    pub locations: Vec<FsLocation4>,
}

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

/// Values for fattr4_fh_expire_type

/// Filehandle never expires
pub const FH4_PERSISTENT: u32 = 0x00000000;

/// Filehandle does not expire while file is open
pub const FH4_NOEXPIRE_WITH_OPEN: u32 = 0x00000001;

/// Filehandle may expire at any time
pub const FH4_VOLATILE_ANY: u32 = 0x00000002;

/// Filehandle may expire on filesystem migration
pub const FH4_VOL_MIGRATION: u32 = 0x00000004;

/// Filehandle may expire on rename
pub const FH4_VOL_RENAME: u32 = 0x00000008;

/// RFC7531: fattr4_supported_attrs
/// Bitmap of supported file attributes.
pub type Fattr4SupportedAttrs = Bitmap4;

/// RFC7531: fattr4_type
/// File type attribute.
pub type Fattr4Type = Ftype4;

/// RFC7531: fattr4_fh_expire_type
/// Filehandle expiration type bitmask.
pub type Fattr4FhExpireType = u32;

/// RFC7531: fattr4_change
/// Change identifier used for cache consistency.
pub type Fattr4Change = ChangeId4;

/// RFC7531: fattr4_size
/// File size in bytes.
pub type Fattr4Size = u64;

/// RFC7531: fattr4_link_support
/// Indicates whether hard links are supported.
/// In Rust this can be represented directly as `bool`,
/// so no dedicated typedef is required.

/// RFC7531: fattr4_symlink_support
/// Indicates whether symbolic links are supported.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_named_attr
/// Indicates support for named attributes.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_fsid
/// File system identifier.
pub type Fattr4FsId = Fsid4;

/// RFC7531: fattr4_unique_handles
/// Indicates whether filehandles are unique.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_lease_time
/// Lease time for stateful operations.
pub type Fattr4LeaseTime = NfsLease4;

/// RFC7531: fattr4_rdattr_error
/// Error value returned when attribute retrieval fails.
pub type Fattr4RdAttrError = NfsStat4;

/// RFC7531: fattr4_acl
/// Access control list attribute.
pub type Fattr4Acl = Vec<NfsAce4>;

/// RFC7531: fattr4_aclsupport
/// ACL support flags.
pub type Fattr4AclSupport = u32;

/// RFC7531: fattr4_archive
/// Archive attribute flag.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_cansettime
/// Indicates whether timestamps can be explicitly set.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_case_insensitive
/// Indicates whether filesystem is case-insensitive.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_case_preserving
/// Indicates whether filesystem preserves case.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_chown_restricted
/// Indicates whether ownership changes are restricted.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_fileid
/// Unique identifier for the file.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_files_avail
/// Available file slots in filesystem.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_filehandle
/// Filehandle attribute.
pub type Fattr4Filehandle = NfsFh4;

/// RFC7531: fattr4_files_free
/// Free file slots.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_files_total
/// Total file slots.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_fs_locations
/// Filesystem location information for migration/replication.
pub type Fattr4FsLocations = FsLocations4;

/// RFC7531: fattr4_hidden
/// Indicates hidden file attribute.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_homogeneous
/// Indicates homogeneous filesystem.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_maxfilesize
/// Maximum supported file size.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_maxlink
/// Maximum number of hard links.
/// Rust uses `u32` directly.

/// RFC7531: fattr4_maxname
/// Maximum filename length.
/// Rust uses `u32` directly.

/// RFC7531: fattr4_maxread
/// Maximum read size supported.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_maxwrite
/// Maximum write size supported.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_mimetype
/// MIME type attribute.
/// Represented as `String` in Rust.

/// RFC7531: fattr4_mode
/// POSIX mode bits.
pub type Fattr4Mode = Mode4;

/// RFC7531: fattr4_mounted_on_fileid
/// File identifier of the mount point.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_no_trunc
/// Indicates names are not truncated.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_numlinks
/// Number of hard links.
/// Rust uses `u32` directly.

/// RFC7531: fattr4_owner
/// File owner name.
/// Represented as `String` in Rust.

/// RFC7531: fattr4_owner_group
/// File group owner name.
/// Represented as `String` in Rust.

/// RFC7531: fattr4_quota_avail_hard
/// Hard quota limit.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_quota_avail_soft
/// Soft quota limit.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_quota_used
/// Quota currently used.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_rawdev
/// Device numbers for special files.
pub type Fattr4RawDev = SpecData4;

/// RFC7531: fattr4_space_avail
/// Available disk space.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_space_free
/// Free disk space.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_space_total
/// Total disk space.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_space_used
/// Used disk space.
/// Rust uses `u64` directly.

/// RFC7531: fattr4_system
/// Indicates system file.
/// Represented directly as `bool` in Rust.

/// RFC7531: fattr4_time_access
/// Last access time.
/// Represented directly as `NfsTime4` in Rust.

/// RFC7531: fattr4_time_access_set
/// Used when setting access time.
/// Represented directly as `SetTime4` in Rust.

/// RFC7531: fattr4_time_backup
/// Backup time attribute.
/// Represented directly as `NfsTime4` in Rust.

/// RFC7531: fattr4_time_create
/// File creation time.
/// Represented directly as `NfsTime4` in Rust.

/// RFC7531: fattr4_time_delta
/// Minimum timestamp granularity.
/// Represented directly as `NfsTime4` in Rust.

/// RFC7531: fattr4_time_metadata
/// Last metadata change time.
/// Represented directly as `NfsTime4` in Rust.

/// RFC7531: fattr4_time_modify
/// Last modification time.
/// Represented directly as `NfsTime4` in Rust.

/// RFC7531: fattr4_time_modify_set
/// Used when setting modification time.
/// Represented directly as `SetTime4` in Rust.

/// RFC7531: Mandatory attribute identifiers.
///
/// These values represent attribute IDs used in the attribute bitmap
/// (`bitmap4`) and during attribute encoding/decoding.

/// Supported attributes bitmap.
pub const FATTR4_SUPPORTED_ATTRS: u32 = 0;

/// File type attribute.
pub const FATTR4_TYPE: u32 = 1;

/// Filehandle expiration type.
pub const FATTR4_FH_EXPIRE_TYPE: u32 = 2;

/// Change attribute (cache consistency).
pub const FATTR4_CHANGE: u32 = 3;

/// File size in bytes.
pub const FATTR4_SIZE: u32 = 4;

/// Indicates whether hard links are supported.
pub const FATTR4_LINK_SUPPORT: u32 = 5;

/// Indicates whether symbolic links are supported.
pub const FATTR4_SYMLINK_SUPPORT: u32 = 6;

/// Indicates whether named attributes are supported.
pub const FATTR4_NAMED_ATTR: u32 = 7;

/// Filesystem identifier.
pub const FATTR4_FSID: u32 = 8;

/// Indicates whether filehandles are unique.
pub const FATTR4_UNIQUE_HANDLES: u32 = 9;

/// Lease time for stateful operations.
pub const FATTR4_LEASE_TIME: u32 = 10;

/// Error attribute returned when attribute retrieval fails.
pub const FATTR4_RDATTR_ERROR: u32 = 11;

/// Filehandle attribute.
pub const FATTR4_FILEHANDLE: u32 = 19;

/// RFC7531: Recommended attribute identifiers.
///
/// These constants represent attribute IDs used in the attribute bitmap
/// (`bitmap4`). They are used when encoding and decoding attribute lists.

pub const FATTR4_ACL: u32 = 12;
pub const FATTR4_ACLSUPPORT: u32 = 13;
pub const FATTR4_ARCHIVE: u32 = 14;
pub const FATTR4_CANSETTIME: u32 = 15;
pub const FATTR4_CASE_INSENSITIVE: u32 = 16;
pub const FATTR4_CASE_PRESERVING: u32 = 17;
pub const FATTR4_CHOWN_RESTRICTED: u32 = 18;

pub const FATTR4_FILEID: u32 = 20;
pub const FATTR4_FILES_AVAIL: u32 = 21;
pub const FATTR4_FILES_FREE: u32 = 22;
pub const FATTR4_FILES_TOTAL: u32 = 23;

pub const FATTR4_FS_LOCATIONS: u32 = 24;

pub const FATTR4_HIDDEN: u32 = 25;
pub const FATTR4_HOMOGENEOUS: u32 = 26;

pub const FATTR4_MAXFILESIZE: u32 = 27;
pub const FATTR4_MAXLINK: u32 = 28;
pub const FATTR4_MAXNAME: u32 = 29;

pub const FATTR4_MAXREAD: u32 = 30;
pub const FATTR4_MAXWRITE: u32 = 31;

pub const FATTR4_MIMETYPE: u32 = 32;

pub const FATTR4_MODE: u32 = 33;

pub const FATTR4_NO_TRUNC: u32 = 34;

pub const FATTR4_NUMLINKS: u32 = 35;

pub const FATTR4_OWNER: u32 = 36;
pub const FATTR4_OWNER_GROUP: u32 = 37;

pub const FATTR4_QUOTA_AVAIL_HARD: u32 = 38;
pub const FATTR4_QUOTA_AVAIL_SOFT: u32 = 39;
pub const FATTR4_QUOTA_USED: u32 = 40;

pub const FATTR4_RAWDEV: u32 = 41;

pub const FATTR4_SPACE_AVAIL: u32 = 42;
pub const FATTR4_SPACE_FREE: u32 = 43;
pub const FATTR4_SPACE_TOTAL: u32 = 44;
pub const FATTR4_SPACE_USED: u32 = 45;

pub const FATTR4_SYSTEM: u32 = 46;

pub const FATTR4_TIME_ACCESS: u32 = 47;
pub const FATTR4_TIME_ACCESS_SET: u32 = 48;
pub const FATTR4_TIME_BACKUP: u32 = 49;
pub const FATTR4_TIME_CREATE: u32 = 50;
pub const FATTR4_TIME_DELTA: u32 = 51;
pub const FATTR4_TIME_METADATA: u32 = 52;
pub const FATTR4_TIME_MODIFY: u32 = 53;
pub const FATTR4_TIME_MODIFY_SET: u32 = 54;

pub const FATTR4_MOUNTED_ON_FILEID: u32 = 55;

/// RFC7531: fattr4
///
/// File attribute container.
/// `attrmask` indicates which attributes are present.
/// `attr_vals` contains the XDR-encoded attribute values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fattr4 {
    pub attrmask: Bitmap4,
    pub attr_vals: Attrlist4,
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

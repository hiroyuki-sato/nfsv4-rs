#![allow(dead_code)]

use super::*;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

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
pub type Fattr4RdAttrError = Stat4;

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

impl Fattr4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let attrmask = Bitmap4::decode(r)?;
        let attr_vals = r.read_opaque()?;
        Ok(Self {
            attrmask,
            attr_vals,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.attrmask.encode(w)?;
        w.write_opaque(&self.attr_vals)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xdr_rs::reader::XdrReader;
    use xdr_rs::writer::XdrWriter;

    #[test]
    fn test_fattr4_encode_decode() {
        let mut attrmask = Bitmap4::new();
        attrmask.insert(0);
        attrmask.insert(31);
        attrmask.insert(32);

        let fattr = Fattr4 {
            attrmask,
            attr_vals: vec![0x11, 0x22, 0x33, 0x44],
        };

        let mut w = XdrWriter::new();
        fattr.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Fattr4::decode(&mut r).unwrap();

        assert_eq!(fattr, decoded);
    }

    #[test]
    fn test_fattr4_encode_decode_empty() {
        let fattr = Fattr4 {
            attrmask: Bitmap4::new(),
            attr_vals: Vec::new(),
        };

        let mut w = XdrWriter::new();
        fattr.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Fattr4::decode(&mut r).unwrap();

        assert_eq!(fattr, decoded);
    }
}

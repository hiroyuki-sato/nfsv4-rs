#![allow(dead_code)]

use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC7531: nfsstat4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NfsStat4 {
    /// NFS4_OK - everything is okay
    Ok = 0,

    /// NFS4ERR_PERM - caller not privileged
    Perm = 1,

    /// NFS4ERR_NOENT - no such file/directory
    NoEnt = 2,

    /// NFS4ERR_IO - hard I/O error
    Io = 5,

    /// NFS4ERR_NXIO - no such device
    NxIo = 6,

    /// NFS4ERR_ACCESS - access denied
    Access = 13,

    /// NFS4ERR_EXIST - file already exists
    Exist = 17,

    /// NFS4ERR_XDEV - different file systems
    XDev = 18,

    /// Unused / Reserved   _Unused1 = 19,

    /// NFS4ERR_NOTDIR - should be a directory
    NotDir = 20,

    /// NFS4ERR_ISDIR - should not be directory
    IsDir = 21,

    /// NFS4ERR_INVAL - invalid argument
    Inval = 22,

    /// NFS4ERR_FBIG - file exceeds server max
    FBig = 27,

    /// NFS4ERR_NOSPC - no space on file system
    NoSpc = 28,

    /// NFS4ERR_ROFS - read-only file system
    Rofs = 30,

    /// NFS4ERR_MLINK - too many hard links
    Mlink = 31,

    /// NFS4ERR_NAMETOOLONG - name exceeds server max
    NameTooLong = 63,

    /// NFS4ERR_NOTEMPTY - directory not empty
    NotEmpty = 66,

    /// NFS4ERR_DQUOT - hard quota limit reached
    Dquot = 69,

    /// NFS4ERR_STALE - file no longer exists
    Stale = 70,

    /// NFS4ERR_BADHANDLE - Illegal filehandle
    BadHandle = 10001,

    /// NFS4ERR_BAD_COOKIE - READDIR cookie is stale
    BadCookie = 10003,

    /// NFS4ERR_NOTSUPP - operation not supported
    NotSupp = 10004,

    /// NFS4ERR_TOOSMALL - response limit exceeded
    TooSmall = 10005,

    /// NFS4ERR_SERVERFAULT - undefined server error
    ServerFault = 10006,

    /// NFS4ERR_BADTYPE - type invalid for CREATE
    BadType = 10007,

    /// NFS4ERR_DELAY - file "busy" - retry
    Delay = 10008,

    /// NFS4ERR_SAME - nverify says attrs same
    Same = 10009,

    /// NFS4ERR_DENIED - lock unavailable
    Denied = 10010,

    /// NFS4ERR_EXPIRED - lock lease expired
    Expired = 10011,

    /// NFS4ERR_LOCKED - I/O failed due to lock
    Locked = 10012,

    /// NFS4ERR_GRACE - in grace period
    Grace = 10013,

    /// NFS4ERR_FHEXPIRED - filehandle expired
    FhExpired = 10014,

    /// NFS4ERR_SHARE_DENIED - share reserve denied
    ShareDenied = 10015,

    /// NFS4ERR_WRONGSEC - wrong security flavor
    WrongSec = 10016,

    /// NFS4ERR_CLID_INUSE - clientid in use
    ClidInUse = 10017,

    /// NFS4ERR_RESOURCE - resource exhaustion
    Resource = 10018,

    /// NFS4ERR_MOVED - file system relocated
    Moved = 10019,

    /// NFS4ERR_NOFILEHANDLE - current FH is not set
    NoFileHandle = 10020,

    /// NFS4ERR_MINOR_VERS_MISMATCH - minor vers not supp
    MinorVersMismatch = 10021,

    /// NFS4ERR_STALE_CLIENTID - server has rebooted
    StaleClientId = 10022,

    /// NFS4ERR_STALE_STATEID - server has rebooted
    StaleStateId = 10023,

    /// NFS4ERR_OLD_STATEID - state is out of sync
    OldStateId = 10024,

    /// NFS4ERR_BAD_STATEID - incorrect stateid
    BadStateId = 10025,

    /// NFS4ERR_BAD_SEQID - request is out of seq.
    BadSeqId = 10026,

    /// NFS4ERR_NOT_SAME - verify - attrs not same
    NotSame = 10027,

    /// NFS4ERR_LOCK_RANGE - lock range not supported
    LockRange = 10028,

    /// NFS4ERR_SYMLINK - should be file/directory
    Symlink = 10029,

    /// NFS4ERR_RESTOREFH - no saved filehandle
    RestoreFh = 10030,

    /// NFS4ERR_LEASE_MOVED - some file system moved
    LeaseMoved = 10031,

    /// NFS4ERR_ATTRNOTSUPP - recommended attr not supported
    AttrNotSupp = 10032,

    /// NFS4ERR_NO_GRACE - reclaim outside of grace
    NoGrace = 10033,

    /// NFS4ERR_RECLAIM_BAD - reclaim error at server
    ReclaimBad = 10034,

    /// NFS4ERR_RECLAIM_CONFLICT - conflict on reclaim
    ReclaimConflict = 10035,

    /// NFS4ERR_BADXDR - XDR decode failed
    BadXdr = 10036,

    /// NFS4ERR_LOCKS_HELD - file locks held at CLOSE
    LocksHeld = 10037,

    /// NFS4ERR_OPENMODE - conflict in OPEN and I/O
    OpenMode = 10038,

    /// NFS4ERR_BADOWNER - owner translation bad
    BadOwner = 10039,

    /// NFS4ERR_BADCHAR - UTF-8 char not supported
    BadChar = 10040,

    /// NFS4ERR_BADNAME - name not supported
    BadName = 10041,

    /// NFS4ERR_BAD_RANGE - lock range not supported
    BadRange = 10042,

    /// NFS4ERR_LOCK_NOTSUPP - no atomic up/downgrade
    LockNotSupp = 10043,

    /// NFS4ERR_OP_ILLEGAL - undefined operation
    OpIllegal = 10044,

    /// NFS4ERR_DEADLOCK - file locking deadlock
    Deadlock = 10045,

    /// NFS4ERR_FILE_OPEN - open file blocks op.
    FileOpen = 10046,

    /// NFS4ERR_ADMIN_REVOKED - lock-owner state revoked
    AdminRevoked = 10047,

    /// NFS4ERR_CB_PATH_DOWN - callback path down
    CbPathDown = 10048,
}

impl TryFrom<i32> for NfsStat4 {
    type Error = Nfsv4Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NfsStat4::Ok),
            1 => Ok(NfsStat4::Perm),
            2 => Ok(NfsStat4::NoEnt),
            5 => Ok(NfsStat4::Io),
            6 => Ok(NfsStat4::NxIo),
            13 => Ok(NfsStat4::Access),
            17 => Ok(NfsStat4::Exist),
            18 => Ok(NfsStat4::XDev),
            20 => Ok(NfsStat4::NotDir),
            21 => Ok(NfsStat4::IsDir),
            22 => Ok(NfsStat4::Inval),
            27 => Ok(NfsStat4::FBig),
            28 => Ok(NfsStat4::NoSpc),
            30 => Ok(NfsStat4::Rofs),
            31 => Ok(NfsStat4::Mlink),
            63 => Ok(NfsStat4::NameTooLong),
            66 => Ok(NfsStat4::NotEmpty),
            69 => Ok(NfsStat4::Dquot),
            70 => Ok(NfsStat4::Stale),
            10001 => Ok(NfsStat4::BadHandle),
            10003 => Ok(NfsStat4::BadCookie),
            10004 => Ok(NfsStat4::NotSupp),
            10005 => Ok(NfsStat4::TooSmall),
            10006 => Ok(NfsStat4::ServerFault),
            10007 => Ok(NfsStat4::BadType),
            10008 => Ok(NfsStat4::Delay),
            10009 => Ok(NfsStat4::Same),
            10010 => Ok(NfsStat4::Denied),
            10011 => Ok(NfsStat4::Expired),
            10012 => Ok(NfsStat4::Locked),
            10013 => Ok(NfsStat4::Grace),
            10014 => Ok(NfsStat4::FhExpired),
            10015 => Ok(NfsStat4::ShareDenied),
            10016 => Ok(NfsStat4::WrongSec),
            10017 => Ok(NfsStat4::ClidInUse),
            10018 => Ok(NfsStat4::Resource),
            10019 => Ok(NfsStat4::Moved),
            10020 => Ok(NfsStat4::NoFileHandle),
            10021 => Ok(NfsStat4::MinorVersMismatch),
            10022 => Ok(NfsStat4::StaleClientId),
            10023 => Ok(NfsStat4::StaleStateId),
            10024 => Ok(NfsStat4::OldStateId),
            10025 => Ok(NfsStat4::BadStateId),
            10026 => Ok(NfsStat4::BadSeqId),
            10027 => Ok(NfsStat4::NotSame),
            10028 => Ok(NfsStat4::LockRange),
            10029 => Ok(NfsStat4::Symlink),
            10030 => Ok(NfsStat4::RestoreFh),
            10031 => Ok(NfsStat4::LeaseMoved),
            10032 => Ok(NfsStat4::AttrNotSupp),
            10033 => Ok(NfsStat4::NoGrace),
            10034 => Ok(NfsStat4::ReclaimBad),
            10035 => Ok(NfsStat4::ReclaimConflict),
            10036 => Ok(NfsStat4::BadXdr),
            10037 => Ok(NfsStat4::LocksHeld),
            10038 => Ok(NfsStat4::OpenMode),
            10039 => Ok(NfsStat4::BadOwner),
            10040 => Ok(NfsStat4::BadChar),
            10041 => Ok(NfsStat4::BadName),
            10042 => Ok(NfsStat4::BadRange),
            10043 => Ok(NfsStat4::LockNotSupp),
            10044 => Ok(NfsStat4::OpIllegal),
            10045 => Ok(NfsStat4::Deadlock),
            10046 => Ok(NfsStat4::FileOpen),
            10047 => Ok(NfsStat4::AdminRevoked),
            10048 => Ok(NfsStat4::CbPathDown),
            _ => Err(Nfsv4Error::InvalidNfsStatus(value)),
        }
    }
}

impl NfsStat4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let value = r.read_i32()?;
        Self::try_from(value)
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(*self as i32)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fstat4_try_from() {
        assert!(matches!(NfsStat4::try_from(0).unwrap(), NfsStat4::Ok));
        assert!(matches!(NfsStat4::try_from(1).unwrap(), NfsStat4::Perm));
        assert!(matches!(NfsStat4::try_from(2).unwrap(), NfsStat4::NoEnt));
        assert!(matches!(NfsStat4::try_from(5).unwrap(), NfsStat4::Io));
        assert!(matches!(NfsStat4::try_from(6).unwrap(), NfsStat4::NxIo));
        assert!(matches!(NfsStat4::try_from(13).unwrap(), NfsStat4::Access));
        assert!(matches!(NfsStat4::try_from(17).unwrap(), NfsStat4::Exist));
        assert!(matches!(NfsStat4::try_from(18).unwrap(), NfsStat4::XDev));
        assert!(matches!(NfsStat4::try_from(20).unwrap(), NfsStat4::NotDir));
        assert!(matches!(NfsStat4::try_from(21).unwrap(), NfsStat4::IsDir));
        assert!(matches!(NfsStat4::try_from(22).unwrap(), NfsStat4::Inval));
        assert!(matches!(NfsStat4::try_from(27).unwrap(), NfsStat4::FBig));
        assert!(matches!(NfsStat4::try_from(28).unwrap(), NfsStat4::NoSpc));
        assert!(matches!(NfsStat4::try_from(30).unwrap(), NfsStat4::Rofs));
        assert!(matches!(NfsStat4::try_from(31).unwrap(), NfsStat4::Mlink));
        assert!(matches!(
            NfsStat4::try_from(63).unwrap(),
            NfsStat4::NameTooLong
        ));
        assert!(matches!(
            NfsStat4::try_from(66).unwrap(),
            NfsStat4::NotEmpty
        ));
        assert!(matches!(NfsStat4::try_from(69).unwrap(), NfsStat4::Dquot));
        assert!(matches!(NfsStat4::try_from(70).unwrap(), NfsStat4::Stale));

        assert!(matches!(
            NfsStat4::try_from(10001).unwrap(),
            NfsStat4::BadHandle
        ));
        assert!(matches!(
            NfsStat4::try_from(10003).unwrap(),
            NfsStat4::BadCookie
        ));
        assert!(matches!(
            NfsStat4::try_from(10004).unwrap(),
            NfsStat4::NotSupp
        ));
        assert!(matches!(
            NfsStat4::try_from(10005).unwrap(),
            NfsStat4::TooSmall
        ));
        assert!(matches!(
            NfsStat4::try_from(10006).unwrap(),
            NfsStat4::ServerFault
        ));
        assert!(matches!(
            NfsStat4::try_from(10007).unwrap(),
            NfsStat4::BadType
        ));
        assert!(matches!(
            NfsStat4::try_from(10008).unwrap(),
            NfsStat4::Delay
        ));
        assert!(matches!(NfsStat4::try_from(10009).unwrap(), NfsStat4::Same));
        assert!(matches!(
            NfsStat4::try_from(10010).unwrap(),
            NfsStat4::Denied
        ));
        assert!(matches!(
            NfsStat4::try_from(10011).unwrap(),
            NfsStat4::Expired
        ));
        assert!(matches!(
            NfsStat4::try_from(10012).unwrap(),
            NfsStat4::Locked
        ));
        assert!(matches!(
            NfsStat4::try_from(10013).unwrap(),
            NfsStat4::Grace
        ));
        assert!(matches!(
            NfsStat4::try_from(10014).unwrap(),
            NfsStat4::FhExpired
        ));
        assert!(matches!(
            NfsStat4::try_from(10015).unwrap(),
            NfsStat4::ShareDenied
        ));
        assert!(matches!(
            NfsStat4::try_from(10016).unwrap(),
            NfsStat4::WrongSec
        ));
        assert!(matches!(
            NfsStat4::try_from(10017).unwrap(),
            NfsStat4::ClidInUse
        ));
        assert!(matches!(
            NfsStat4::try_from(10018).unwrap(),
            NfsStat4::Resource
        ));
        assert!(matches!(
            NfsStat4::try_from(10019).unwrap(),
            NfsStat4::Moved
        ));
        assert!(matches!(
            NfsStat4::try_from(10020).unwrap(),
            NfsStat4::NoFileHandle
        ));
        assert!(matches!(
            NfsStat4::try_from(10021).unwrap(),
            NfsStat4::MinorVersMismatch
        ));
        assert!(matches!(
            NfsStat4::try_from(10022).unwrap(),
            NfsStat4::StaleClientId
        ));
        assert!(matches!(
            NfsStat4::try_from(10023).unwrap(),
            NfsStat4::StaleStateId
        ));
        assert!(matches!(
            NfsStat4::try_from(10024).unwrap(),
            NfsStat4::OldStateId
        ));

        assert!(matches!(
            NfsStat4::try_from(10025).unwrap(),
            NfsStat4::BadStateId
        ));
        assert!(matches!(
            NfsStat4::try_from(10026).unwrap(),
            NfsStat4::BadSeqId
        ));
        assert!(matches!(
            NfsStat4::try_from(10027).unwrap(),
            NfsStat4::NotSame
        ));
        assert!(matches!(
            NfsStat4::try_from(10028).unwrap(),
            NfsStat4::LockRange
        ));
        assert!(matches!(
            NfsStat4::try_from(10029).unwrap(),
            NfsStat4::Symlink
        ));
        assert!(matches!(
            NfsStat4::try_from(10030).unwrap(),
            NfsStat4::RestoreFh
        ));
        assert!(matches!(
            NfsStat4::try_from(10031).unwrap(),
            NfsStat4::LeaseMoved
        ));
        assert!(matches!(
            NfsStat4::try_from(10032).unwrap(),
            NfsStat4::AttrNotSupp
        ));
        assert!(matches!(
            NfsStat4::try_from(10033).unwrap(),
            NfsStat4::NoGrace
        ));
        assert!(matches!(
            NfsStat4::try_from(10034).unwrap(),
            NfsStat4::ReclaimBad
        ));
        assert!(matches!(
            NfsStat4::try_from(10035).unwrap(),
            NfsStat4::ReclaimConflict
        ));
        assert!(matches!(
            NfsStat4::try_from(10036).unwrap(),
            NfsStat4::BadXdr
        ));
        assert!(matches!(
            NfsStat4::try_from(10037).unwrap(),
            NfsStat4::LocksHeld
        ));
        assert!(matches!(
            NfsStat4::try_from(10038).unwrap(),
            NfsStat4::OpenMode
        ));
        assert!(matches!(
            NfsStat4::try_from(10039).unwrap(),
            NfsStat4::BadOwner
        ));
        assert!(matches!(
            NfsStat4::try_from(10040).unwrap(),
            NfsStat4::BadChar
        ));
        assert!(matches!(
            NfsStat4::try_from(10041).unwrap(),
            NfsStat4::BadName
        ));
        assert!(matches!(
            NfsStat4::try_from(10042).unwrap(),
            NfsStat4::BadRange
        ));
        assert!(matches!(
            NfsStat4::try_from(10043).unwrap(),
            NfsStat4::LockNotSupp
        ));
        assert!(matches!(
            NfsStat4::try_from(10044).unwrap(),
            NfsStat4::OpIllegal
        ));
        assert!(matches!(
            NfsStat4::try_from(10045).unwrap(),
            NfsStat4::Deadlock
        ));
        assert!(matches!(
            NfsStat4::try_from(10046).unwrap(),
            NfsStat4::FileOpen
        ));
        assert!(matches!(
            NfsStat4::try_from(10047).unwrap(),
            NfsStat4::AdminRevoked
        ));
        assert!(matches!(
            NfsStat4::try_from(10048).unwrap(),
            NfsStat4::CbPathDown
        ));
    }

    #[test]
    fn test_encode_decode() {
        let status = NfsStat4::BadStateId;
        let mut w = XdrWriter::new();
        status.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded_status = NfsStat4::decode(&mut r).unwrap();
        assert_eq!(status, decoded_status);
    }
}

#![allow(dead_code)]

use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC7531: Stat4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Stat4 {
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

impl TryFrom<i32> for Stat4 {
    type Error = Nfsv4Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Stat4::Ok),
            1 => Ok(Stat4::Perm),
            2 => Ok(Stat4::NoEnt),
            5 => Ok(Stat4::Io),
            6 => Ok(Stat4::NxIo),
            13 => Ok(Stat4::Access),
            17 => Ok(Stat4::Exist),
            18 => Ok(Stat4::XDev),
            20 => Ok(Stat4::NotDir),
            21 => Ok(Stat4::IsDir),
            22 => Ok(Stat4::Inval),
            27 => Ok(Stat4::FBig),
            28 => Ok(Stat4::NoSpc),
            30 => Ok(Stat4::Rofs),
            31 => Ok(Stat4::Mlink),
            63 => Ok(Stat4::NameTooLong),
            66 => Ok(Stat4::NotEmpty),
            69 => Ok(Stat4::Dquot),
            70 => Ok(Stat4::Stale),
            10001 => Ok(Stat4::BadHandle),
            10003 => Ok(Stat4::BadCookie),
            10004 => Ok(Stat4::NotSupp),
            10005 => Ok(Stat4::TooSmall),
            10006 => Ok(Stat4::ServerFault),
            10007 => Ok(Stat4::BadType),
            10008 => Ok(Stat4::Delay),
            10009 => Ok(Stat4::Same),
            10010 => Ok(Stat4::Denied),
            10011 => Ok(Stat4::Expired),
            10012 => Ok(Stat4::Locked),
            10013 => Ok(Stat4::Grace),
            10014 => Ok(Stat4::FhExpired),
            10015 => Ok(Stat4::ShareDenied),
            10016 => Ok(Stat4::WrongSec),
            10017 => Ok(Stat4::ClidInUse),
            10018 => Ok(Stat4::Resource),
            10019 => Ok(Stat4::Moved),
            10020 => Ok(Stat4::NoFileHandle),
            10021 => Ok(Stat4::MinorVersMismatch),
            10022 => Ok(Stat4::StaleClientId),
            10023 => Ok(Stat4::StaleStateId),
            10024 => Ok(Stat4::OldStateId),
            10025 => Ok(Stat4::BadStateId),
            10026 => Ok(Stat4::BadSeqId),
            10027 => Ok(Stat4::NotSame),
            10028 => Ok(Stat4::LockRange),
            10029 => Ok(Stat4::Symlink),
            10030 => Ok(Stat4::RestoreFh),
            10031 => Ok(Stat4::LeaseMoved),
            10032 => Ok(Stat4::AttrNotSupp),
            10033 => Ok(Stat4::NoGrace),
            10034 => Ok(Stat4::ReclaimBad),
            10035 => Ok(Stat4::ReclaimConflict),
            10036 => Ok(Stat4::BadXdr),
            10037 => Ok(Stat4::LocksHeld),
            10038 => Ok(Stat4::OpenMode),
            10039 => Ok(Stat4::BadOwner),
            10040 => Ok(Stat4::BadChar),
            10041 => Ok(Stat4::BadName),
            10042 => Ok(Stat4::BadRange),
            10043 => Ok(Stat4::LockNotSupp),
            10044 => Ok(Stat4::OpIllegal),
            10045 => Ok(Stat4::Deadlock),
            10046 => Ok(Stat4::FileOpen),
            10047 => Ok(Stat4::AdminRevoked),
            10048 => Ok(Stat4::CbPathDown),
            _ => Err(Nfsv4Error::InvalidNfsStatus(value)),
        }
    }
}

impl Stat4 {
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
        assert!(matches!(Stat4::try_from(0).unwrap(), Stat4::Ok));
        assert!(matches!(Stat4::try_from(1).unwrap(), Stat4::Perm));
        assert!(matches!(Stat4::try_from(2).unwrap(), Stat4::NoEnt));
        assert!(matches!(Stat4::try_from(5).unwrap(), Stat4::Io));
        assert!(matches!(Stat4::try_from(6).unwrap(), Stat4::NxIo));
        assert!(matches!(Stat4::try_from(13).unwrap(), Stat4::Access));
        assert!(matches!(Stat4::try_from(17).unwrap(), Stat4::Exist));
        assert!(matches!(Stat4::try_from(18).unwrap(), Stat4::XDev));
        assert!(matches!(Stat4::try_from(20).unwrap(), Stat4::NotDir));
        assert!(matches!(Stat4::try_from(21).unwrap(), Stat4::IsDir));
        assert!(matches!(Stat4::try_from(22).unwrap(), Stat4::Inval));
        assert!(matches!(Stat4::try_from(27).unwrap(), Stat4::FBig));
        assert!(matches!(Stat4::try_from(28).unwrap(), Stat4::NoSpc));
        assert!(matches!(Stat4::try_from(30).unwrap(), Stat4::Rofs));
        assert!(matches!(Stat4::try_from(31).unwrap(), Stat4::Mlink));
        assert!(matches!(Stat4::try_from(63).unwrap(), Stat4::NameTooLong));
        assert!(matches!(Stat4::try_from(66).unwrap(), Stat4::NotEmpty));
        assert!(matches!(Stat4::try_from(69).unwrap(), Stat4::Dquot));
        assert!(matches!(Stat4::try_from(70).unwrap(), Stat4::Stale));

        assert!(matches!(Stat4::try_from(10001).unwrap(), Stat4::BadHandle));
        assert!(matches!(Stat4::try_from(10003).unwrap(), Stat4::BadCookie));
        assert!(matches!(Stat4::try_from(10004).unwrap(), Stat4::NotSupp));
        assert!(matches!(Stat4::try_from(10005).unwrap(), Stat4::TooSmall));
        assert!(matches!(
            Stat4::try_from(10006).unwrap(),
            Stat4::ServerFault
        ));
        assert!(matches!(Stat4::try_from(10007).unwrap(), Stat4::BadType));
        assert!(matches!(Stat4::try_from(10008).unwrap(), Stat4::Delay));
        assert!(matches!(Stat4::try_from(10009).unwrap(), Stat4::Same));
        assert!(matches!(Stat4::try_from(10010).unwrap(), Stat4::Denied));
        assert!(matches!(Stat4::try_from(10011).unwrap(), Stat4::Expired));
        assert!(matches!(Stat4::try_from(10012).unwrap(), Stat4::Locked));
        assert!(matches!(Stat4::try_from(10013).unwrap(), Stat4::Grace));
        assert!(matches!(Stat4::try_from(10014).unwrap(), Stat4::FhExpired));
        assert!(matches!(
            Stat4::try_from(10015).unwrap(),
            Stat4::ShareDenied
        ));
        assert!(matches!(Stat4::try_from(10016).unwrap(), Stat4::WrongSec));
        assert!(matches!(Stat4::try_from(10017).unwrap(), Stat4::ClidInUse));
        assert!(matches!(Stat4::try_from(10018).unwrap(), Stat4::Resource));
        assert!(matches!(Stat4::try_from(10019).unwrap(), Stat4::Moved));
        assert!(matches!(
            Stat4::try_from(10020).unwrap(),
            Stat4::NoFileHandle
        ));
        assert!(matches!(
            Stat4::try_from(10021).unwrap(),
            Stat4::MinorVersMismatch
        ));
        assert!(matches!(
            Stat4::try_from(10022).unwrap(),
            Stat4::StaleClientId
        ));
        assert!(matches!(
            Stat4::try_from(10023).unwrap(),
            Stat4::StaleStateId
        ));
        assert!(matches!(Stat4::try_from(10024).unwrap(), Stat4::OldStateId));

        assert!(matches!(Stat4::try_from(10025).unwrap(), Stat4::BadStateId));
        assert!(matches!(Stat4::try_from(10026).unwrap(), Stat4::BadSeqId));
        assert!(matches!(Stat4::try_from(10027).unwrap(), Stat4::NotSame));
        assert!(matches!(Stat4::try_from(10028).unwrap(), Stat4::LockRange));
        assert!(matches!(Stat4::try_from(10029).unwrap(), Stat4::Symlink));
        assert!(matches!(Stat4::try_from(10030).unwrap(), Stat4::RestoreFh));
        assert!(matches!(Stat4::try_from(10031).unwrap(), Stat4::LeaseMoved));
        assert!(matches!(
            Stat4::try_from(10032).unwrap(),
            Stat4::AttrNotSupp
        ));
        assert!(matches!(Stat4::try_from(10033).unwrap(), Stat4::NoGrace));
        assert!(matches!(Stat4::try_from(10034).unwrap(), Stat4::ReclaimBad));
        assert!(matches!(
            Stat4::try_from(10035).unwrap(),
            Stat4::ReclaimConflict
        ));
        assert!(matches!(Stat4::try_from(10036).unwrap(), Stat4::BadXdr));
        assert!(matches!(Stat4::try_from(10037).unwrap(), Stat4::LocksHeld));
        assert!(matches!(Stat4::try_from(10038).unwrap(), Stat4::OpenMode));
        assert!(matches!(Stat4::try_from(10039).unwrap(), Stat4::BadOwner));
        assert!(matches!(Stat4::try_from(10040).unwrap(), Stat4::BadChar));
        assert!(matches!(Stat4::try_from(10041).unwrap(), Stat4::BadName));
        assert!(matches!(Stat4::try_from(10042).unwrap(), Stat4::BadRange));
        assert!(matches!(
            Stat4::try_from(10043).unwrap(),
            Stat4::LockNotSupp
        ));
        assert!(matches!(Stat4::try_from(10044).unwrap(), Stat4::OpIllegal));
        assert!(matches!(Stat4::try_from(10045).unwrap(), Stat4::Deadlock));
        assert!(matches!(Stat4::try_from(10046).unwrap(), Stat4::FileOpen));
        assert!(matches!(
            Stat4::try_from(10047).unwrap(),
            Stat4::AdminRevoked
        ));
        assert!(matches!(Stat4::try_from(10048).unwrap(), Stat4::CbPathDown));
    }

    #[test]
    fn test_encode_decode() {
        let status = Stat4::BadStateId;
        let mut w = XdrWriter::new();
        status.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded_status = Stat4::decode(&mut r).unwrap();
        assert_eq!(status, decoded_status);
    }
}

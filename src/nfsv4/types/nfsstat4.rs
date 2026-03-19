#![allow(dead_code)]

use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC7531: nfsstat4 (partial)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NfsStat4 {
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

#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::SESSIONID_SIZE;
use crate::nfsv4::types::SequenceId4;
use crate::nfsv4::types::SessionId4;
use crate::nfsv4::types::SlotId4;
use crate::nfsv4::types::Stat4;
use crate::nfsv4::types::StateId4;

/// RFC8881 Section 18.46.1: SEQUENCE4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence4Args {
    pub sa_sessionid: SessionId4,
    pub sa_sequenceid: SequenceId4,
    pub sa_slotid: SlotId4,
    pub sa_highest_slotid: SlotId4,
    pub sa_cachethis: bool,
}

/// RFC8881 Section 18.46.2: SEQ4_STATUS_*
///
/// Status flags returned in SEQUENCE4resok.sr_status_flags
pub const SEQ4_STATUS_CB_PATH_DOWN: u32 = 0x00000001;
pub const SEQ4_STATUS_CB_GSS_CONTEXTS_EXPIRING: u32 = 0x00000002;
pub const SEQ4_STATUS_CB_GSS_CONTEXTS_EXPIRED: u32 = 0x00000004;
pub const SEQ4_STATUS_EXPIRED_ALL_STATE_REVOKED: u32 = 0x00000008;
pub const SEQ4_STATUS_EXPIRED_SOME_STATE_REVOKED: u32 = 0x00000010;
pub const SEQ4_STATUS_ADMIN_STATE_REVOKED: u32 = 0x00000020;
pub const SEQ4_STATUS_RECALLABLE_STATE_REVOKED: u32 = 0x00000040;
pub const SEQ4_STATUS_LEASE_MOVED: u32 = 0x00000080;
pub const SEQ4_STATUS_RESTART_RECLAIM_NEEDED: u32 = 0x00000100;
pub const SEQ4_STATUS_CB_PATH_DOWN_SESSION: u32 = 0x00000200;
pub const SEQ4_STATUS_BACKCHANNEL_FAULT: u32 = 0x00000400;
pub const SEQ4_STATUS_DEVID_CHANGED: u32 = 0x00000800;
pub const SEQ4_STATUS_DEVID_DELETED: u32 = 0x00001000;

/// RFC8881 Section 18.46.2: SEQUENCE4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence4ResOk {
    pub sr_sessionid: SessionId4,
    pub sr_sequenceid: SequenceId4,
    pub sr_slotid: SlotId4,
    pub sr_highest_slotid: SlotId4,
    pub sr_target_highest_slotid: SlotId4,
    pub sr_status_flags: u32,
}

impl Sequence4ResOk {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let sr_sessionid: SessionId4 = r
            .read_fixed_opaque(SESSIONID_SIZE)?
            .try_into()
            .map_err(|_| Nfsv4Error::InvalidData("expected sessionid4".into()))?;
        let sr_sequenceid = r.read_u32()?;
        let sr_slotid = r.read_u32()?;
        let sr_highest_slotid = r.read_u32()?;
        let sr_target_highest_slotid = r.read_u32()?;
        let sr_status_flags = r.read_u32()?;
        Ok(Self {
            sr_sessionid,
            sr_sequenceid,
            sr_slotid,
            sr_highest_slotid,
            sr_target_highest_slotid,
            sr_status_flags,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_fixed_opaque(&self.sr_sessionid)?;
        w.write_u32(self.sr_sequenceid)?;
        w.write_u32(self.sr_slotid)?;
        w.write_u32(self.sr_highest_slotid)?;
        w.write_u32(self.sr_target_highest_slotid)?;
        w.write_u32(self.sr_status_flags)?;
        Ok(())
    }
}

/// RFC8881 Section 18.46.2: SEQUENCE4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sequence4Res {
    Ok(Sequence4ResOk),
    Err(Stat4),
}

impl Sequence4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let sa_sessionid: SessionId4 = r
            .read_fixed_opaque(SESSIONID_SIZE)?
            .try_into()
            .map_err(|_| Nfsv4Error::InvalidData("expected sessionid4".into()))?;
        let sa_sequenceid = r.read_u32()?;
        let sa_slotid = r.read_u32()?;
        let sa_highest_slotid = r.read_u32()?;
        let sa_cachethis = r.read_boolean()?;
        Ok(Self {
            sa_sessionid,
            sa_sequenceid,
            sa_slotid,
            sa_highest_slotid,
            sa_cachethis,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_fixed_opaque(&self.sa_sessionid)?;
        w.write_u32(self.sa_sequenceid)?;
        w.write_u32(self.sa_slotid)?;
        w.write_u32(self.sa_highest_slotid)?;
        w.write_boolean(self.sa_cachethis)?;
        Ok(())
    }
}

impl Sequence4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let status = Stat4::decode(r)?;
        match status {
            Stat4::Ok => Ok(Self::Ok(Sequence4ResOk::decode(r)?)),
            err => Ok(Self::Err(err)),
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::Ok(resok) => {
                Stat4::Ok.encode(w)?;
                resok.encode(w)?;
            }
            Self::Err(err) => {
                err.encode(w)?;
            }
        }
        Ok(())
    }
}

#[test]
fn test_sequence4args_encode_decode() {
    let args = Sequence4Args {
        sa_sessionid: [0x11; SESSIONID_SIZE],
        sa_sequenceid: 1,
        sa_slotid: 2,
        sa_highest_slotid: 3,
        sa_cachethis: true,
    };

    let mut w = XdrWriter::new();
    args.encode(&mut w).unwrap();

    let mut r = XdrReader::new(w.as_bytes());
    let decoded = Sequence4Args::decode(&mut r).unwrap();

    assert_eq!(args, decoded);
}

#[test]
fn test_sequence4resok_encode_decode() {
    let resok = Sequence4ResOk {
        sr_sessionid: [0x22; SESSIONID_SIZE],
        sr_sequenceid: 10,
        sr_slotid: 11,
        sr_highest_slotid: 12,
        sr_target_highest_slotid: 13,
        sr_status_flags: SEQ4_STATUS_CB_PATH_DOWN | SEQ4_STATUS_DEVID_CHANGED,
    };

    let mut w = XdrWriter::new();
    resok.encode(&mut w).unwrap();

    let mut r = XdrReader::new(w.as_bytes());
    let decoded = Sequence4ResOk::decode(&mut r).unwrap();

    assert_eq!(resok, decoded);
}

#[test]
fn test_sequence4res_ok_encode_decode() {
    let res = Sequence4Res::Ok(Sequence4ResOk {
        sr_sessionid: [0x33; SESSIONID_SIZE],
        sr_sequenceid: 100,
        sr_slotid: 101,
        sr_highest_slotid: 102,
        sr_target_highest_slotid: 103,
        sr_status_flags: SEQ4_STATUS_BACKCHANNEL_FAULT,
    });

    let mut w = XdrWriter::new();
    res.encode(&mut w).unwrap();

    let mut r = XdrReader::new(w.as_bytes());
    let decoded = Sequence4Res::decode(&mut r).unwrap();

    assert_eq!(res, decoded);
}

#[test]
fn test_sequence4res_err_encode_decode() {
    let res = Sequence4Res::Err(Stat4::BadStateId);

    let mut w = XdrWriter::new();
    res.encode(&mut w).unwrap();

    let mut r = XdrReader::new(w.as_bytes());
    let decoded = Sequence4Res::decode(&mut r).unwrap();

    assert_eq!(res, decoded);
}

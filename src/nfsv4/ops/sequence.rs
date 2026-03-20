#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
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

/// RFC8881 Section 18.46.2: SEQUENCE4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sequence4Res {
    Ok(Sequence4ResOk),
    Err(Stat4),
}

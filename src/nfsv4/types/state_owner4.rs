#![allow(dead_code)]

use crate::nfsv4::types::ClientId4;
use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881: state_owner4
///
/// Identifies a state owner (client + opaque owner identifier).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateOwner4 {
    pub clientid: ClientId4,
    pub owner: Vec<u8>,
}

/// RFC8881: lock_owner4
///
/// Identifies a LOCK owner (same structure as state_owner4).
pub type LockOwner4 = StateOwner4;

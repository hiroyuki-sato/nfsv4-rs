#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: CLOSE4args
///
/// Arguments for the CLOSE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Close4Args {
    /// Sequence ID for the open-owner.
    pub seqid: SeqId4,

    /// State ID returned by OPEN.
    pub open_stateid: StateId4,
}

/// RFC7531: CLOSE4res
///
/// Result of the CLOSE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Close4Res {
    /// Operation succeeded.
    Ok(StateId4),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

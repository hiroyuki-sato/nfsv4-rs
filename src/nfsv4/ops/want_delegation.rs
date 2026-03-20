#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::OpenDelegation4;
use crate::nfsv4::ops::OpenDelegationType4;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.49.1: deleg_claim4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelegClaim4 {
    /// CLAIM_FH
    ClaimFh,

    /// CLAIM_DELEG_PREV_FH
    ClaimDelegPrevFh,

    /// CLAIM_PREVIOUS
    ClaimPrevious(OpenDelegationType4),
}

/// RFC8881 Section 18.49.1: WANT_DELEGATION4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WantDelegation4Args {
    pub wda_want: u32,
    pub wda_claim: DelegClaim4,
}

/// RFC8881 Section 18.49.2: WANT_DELEGATION4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WantDelegation4Res {
    Ok(OpenDelegation4),
    Err(Stat4),
}

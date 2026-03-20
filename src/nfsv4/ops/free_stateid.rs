use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::Stat4;
use crate::nfsv4::types::StateId4;

/// RFC8881 Section 18.38.1: FREE_STATEID4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeStateId4Args {
    pub fsa_stateid: StateId4,
}

/// RFC8881 Section 18.38.2: FREE_STATEID4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeStateId4Res {
    pub fsr_status: Stat4,
}

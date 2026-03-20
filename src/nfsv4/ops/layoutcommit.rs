#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::LayoutUpdate4;
use crate::nfsv4::types::Length4;
use crate::nfsv4::types::NfsTime4;
use crate::nfsv4::types::Offset4;
use crate::nfsv4::types::Stat4;
use crate::nfsv4::types::StateId4;

/// RFC8881 Section 18.42.1: newtime4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewTime4 {
    Changed(NfsTime4),
    Unchanged,
}

/// RFC8881 Section 18.42.1: newoffset4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewOffset4 {
    Offset(Offset4),
    None,
}

/// RFC8881 Section 18.42.1: LAYOUTCOMMIT4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutCommit4Args {
    /// CURRENT_FH: file
    pub loca_offset: Offset4,
    pub loca_length: Length4,
    pub loca_reclaim: bool,
    pub loca_stateid: StateId4,
    pub loca_last_write_offset: NewOffset4,
    pub loca_time_modify: NewTime4,
    pub loca_layoutupdate: LayoutUpdate4,
}

/// RFC8881 Section 18.42.2: newsize4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewSize4 {
    Changed(Length4),
    Unchanged,
}

/// RFC8881 Section 18.42.2: LAYOUTCOMMIT4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutCommit4ResOk {
    pub locr_newsize: NewSize4,
}

/// RFC8881 Section 18.42.2: LAYOUTCOMMIT4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutCommit4Res {
    Ok(LayoutCommit4ResOk),
    Err(Stat4),
}

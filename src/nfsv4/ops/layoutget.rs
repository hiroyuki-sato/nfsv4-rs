#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::layoutcommit::NewOffset4;
use crate::nfsv4::ops::layoutcommit::NewSize4;
use crate::nfsv4::ops::layoutcommit::NewTime4;
use crate::nfsv4::types::Count4;
use crate::nfsv4::types::Layout4;
use crate::nfsv4::types::LayoutIOMode4;
use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Length4;
use crate::nfsv4::types::Offset4;
use crate::nfsv4::types::Stat4;
use crate::nfsv4::types::StateId4;

/// RFC8881 Section 18.43.1: LAYOUTGET4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutGet4Args {
    /// CURRENT_FH: file
    pub loga_signal_layout_avail: bool,
    pub loga_layout_type: LayoutType4,
    pub loga_iomode: LayoutIOMode4,
    pub loga_offset: Offset4,
    pub loga_length: Length4,
    pub loga_minlength: Length4,
    pub loga_stateid: StateId4,
    pub loga_maxcount: Count4,
}

/// RFC8881 Section 18.43.2: LAYOUTGET4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutGet4ResOk {
    pub logr_return_on_close: bool,
    pub logr_stateid: StateId4,
    pub logr_layout: Vec<Layout4>,
}

/// RFC8881 Section 18.43.2: LAYOUTGET4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutGet4Res {
    Ok(LayoutGet4ResOk),
    LayoutTryLater(bool),
    Err(Stat4),
}

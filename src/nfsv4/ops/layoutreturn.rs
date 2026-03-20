#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::LayoutIOMode4;
use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Length4;
use crate::nfsv4::types::Offset4;
use crate::nfsv4::types::Stat4;
use crate::nfsv4::types::StateId4;

/// RFC8881 Section 18.44: LAYOUT4_RET_REC_*
///
/// Constants used for LAYOUTRETURN and CB_LAYOUTRECALL.
pub const LAYOUT4_RET_REC_FILE: u32 = 1;
pub const LAYOUT4_RET_REC_FSID: u32 = 2;
pub const LAYOUT4_RET_REC_ALL: u32 = 3;

/// RFC8881 Section 18.44.1: layoutreturn_type4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LayoutReturnType4 {
    LayoutReturnFile = LAYOUT4_RET_REC_FILE as i32,
    LayoutReturnFsid = LAYOUT4_RET_REC_FSID as i32,
    LayoutReturnAll = LAYOUT4_RET_REC_ALL as i32,
}

/// RFC8881 Section 18.44.1: layoutreturn_file4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutReturnFile4 {
    pub lrf_offset: Offset4,
    pub lrf_length: Length4,
    pub lrf_stateid: StateId4,
    /// layouttype4 specific data
    pub lrf_body: Vec<u8>,
}

/// RFC8881 Section 18.44.1: layoutreturn4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutReturn4 {
    File(LayoutReturnFile4),
    Fsid,
    All,
}

/// RFC8881 Section 18.44.1: LAYOUTRETURN4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutReturn4Args {
    /// CURRENT_FH: file
    pub lora_reclaim: bool,
    pub lora_layout_type: LayoutType4,
    pub lora_iomode: LayoutIOMode4,
    pub lora_layoutreturn: LayoutReturn4,
}

/// RFC8881 Section 18.44.2: layoutreturn_stateid
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutReturnStateId {
    Present(StateId4),
    None,
}

/// RFC8881 Section 18.44.2: LAYOUTRETURN4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutReturn4Res {
    Ok(LayoutReturnStateId),
    Err(Stat4),
}

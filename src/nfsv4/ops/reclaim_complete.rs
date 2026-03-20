#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.51.1: RECLAIM_COMPLETE4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimComplete4Args {
    /// If TRUE, CURRENT_FH identifies the file system
    pub rca_one_fs: bool,
}

/// RFC8881 Section 18.51.2: RECLAIM_COMPLETE4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimComplete4Res {
    pub rcr_status: Stat4,
}

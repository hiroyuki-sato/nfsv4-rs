#![allow(dead_code)]

use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Nfsv4Error;
use crate::nfsv4::types::ThresholdItem4;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.23: mdsthreshold4
///
/// Collection of threshold hint items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MdsThreshold4 {
    /// Threshold hint items
    pub mth_hints: Vec<ThresholdItem4>,
}

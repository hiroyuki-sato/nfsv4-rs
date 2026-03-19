#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: SETATTR4args
///
/// Arguments for the SETATTR operation.
/// CURRENT_FH must refer to the target object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetAttr4Args {
    /// State ID used for attribute changes.
    pub stateid: StateId4,

    /// Attributes to be set on the object.
    pub obj_attributes: Fattr4,
}

/// RFC7531: SETATTR4res
///
/// Result of the SETATTR operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetAttr4Res {
    /// NFS operation status.
    pub status: Stat4,

    /// Bitmap of attributes successfully set.
    pub attrsset: Bitmap4,
}

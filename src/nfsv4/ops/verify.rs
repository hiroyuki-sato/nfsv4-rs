#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: VERIFY4args
///
/// Arguments for the VERIFY operation.
/// CURRENT_FH must refer to the target object.
///
/// The operation succeeds only if the specified attributes
/// match the object's attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verify4Args {
    /// Attributes to compare against the object.
    pub obj_attributes: Fattr4,
}

/// RFC7531: VERIFY4res
///
/// Result of the VERIFY operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verify4Res {
    /// NFS operation status.
    pub status: Stat4,
}

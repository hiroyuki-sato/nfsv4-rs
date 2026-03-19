#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: LOOKUP4args
///
/// Arguments for the LOOKUP operation.
/// CURRENT_FH must refer to a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookup4Args {
    /// Name of the object to look up.
    pub objname: Component4,
}

/// RFC7531: LOOKUP4res
///
/// Result of the LOOKUP operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookup4Res {
    /// NFS operation status.
    pub status: Stat4,
}

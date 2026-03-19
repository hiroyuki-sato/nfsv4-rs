#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: LINK4args
///
/// Arguments for the LINK operation.
/// SAVED_FH must refer to the source object.
/// CURRENT_FH must refer to the target directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link4Args {
    /// New name of the linked object in the target directory.
    pub newname: Component4,
}

/// RFC7531: LINK4resok
///
/// Successful result of the LINK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link4ResOk {
    /// Change information for the target directory.
    pub cinfo: ChangeInfo4,
}

/// RFC7531: LINK4res
///
/// Result of the LINK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Link4Res {
    /// Operation succeeded.
    Ok(Link4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: READLINK4resok
///
/// Successful result of the READLINK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadLink4ResOk {
    /// Symbolic link target.
    pub link: LinkText4,
}

/// RFC7531: READLINK4res
///
/// Result of the READLINK operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadLink4Res {
    /// Operation succeeded.
    Ok(ReadLink4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

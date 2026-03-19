#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: RENEW4args
///
/// Arguments for the RENEW operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renew4Args {
    /// Client identifier to renew.
    pub clientid: ClientId4,
}

/// RFC7531: RENEW4res
///
/// Result of the RENEW operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renew4Res {
    /// NFS operation status.
    pub status: Stat4,
}

#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: DELEGPURGE4args
///
/// Arguments for the DELEGPURGE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegPurge4Args {
    /// Client identifier whose delegations should be purged.
    pub clientid: ClientId4,
}

/// RFC7531: DELEGPURGE4res
///
/// Result of the DELEGPURGE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegPurge4Res {
    /// NFS operation status.
    pub status: Stat4,
}

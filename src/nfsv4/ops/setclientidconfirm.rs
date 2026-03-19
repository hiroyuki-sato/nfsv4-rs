#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: SETCLIENTID_CONFIRM4args
///
/// Arguments for the SETCLIENTID_CONFIRM operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientIdConfirm4Args {
    /// Client identifier returned by SETCLIENTID.
    pub clientid: ClientId4,

    /// Confirmation verifier returned by SETCLIENTID.
    pub setclientid_confirm: Verifier4,
}

/// RFC7531: SETCLIENTID_CONFIRM4res
///
/// Result of the SETCLIENTID_CONFIRM operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientIdConfirm4Res {
    /// NFS operation status.
    pub status: Stat4,
}

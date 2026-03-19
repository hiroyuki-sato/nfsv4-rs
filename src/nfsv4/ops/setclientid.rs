#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: SETCLIENTID4args
///
/// Arguments for the SETCLIENTID operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientId4Args {
    /// Client identification information.
    pub client: NfsClientId4,

    /// Callback program information.
    pub callback: CbClient4,

    /// Client-chosen callback identifier.
    pub callback_ident: u32,
}

/// RFC7531: SETCLIENTID4resok
///
/// Successful result of the SETCLIENTID operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientId4ResOk {
    /// Assigned client identifier.
    pub clientid: ClientId4,

    /// Verifier used in SETCLIENTID_CONFIRM.
    pub setclientid_confirm: Verifier4,
}

/// RFC7531: SETCLIENTID4res
///
/// Result of the SETCLIENTID operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetClientId4Res {
    /// Operation succeeded.
    Ok(SetClientId4ResOk),

    /// Client ID is already in use by another client address.
    ClientIdInUse(ClientAddr4),

    /// Operation failed with another NFS error status.
    Err(Stat4),
}

#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::ClientId4;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.50.1: DESTROY_CLIENTID4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestroyClientId4Args {
    pub dca_clientid: ClientId4,
}

/// RFC8881 Section 18.50.2: DESTROY_CLIENTID4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestroyClientId4Res {
    pub dcr_status: Stat4,
}

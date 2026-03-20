#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::SessionId4;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.37.1: DESTROY_SESSION4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestroySession4Args {
    pub dsa_sessionid: SessionId4,
}

/// RFC8881 Section 18.37.2: DESTROY_SESSION4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestroySession4Res {
    pub dsr_status: Stat4,
}

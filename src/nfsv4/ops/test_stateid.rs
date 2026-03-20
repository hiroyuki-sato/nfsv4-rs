#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::Stat4;
use crate::nfsv4::types::StateId4;

/// RFC8881 Section 18.48.1: TEST_STATEID4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestStateId4Args {
    pub ts_stateids: Vec<StateId4>,
}

/// RFC8881 Section 18.48.2: TEST_STATEID4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestStateId4ResOk {
    pub tsr_status_codes: Vec<Stat4>,
}

/// RFC8881 Section 18.48.2: TEST_STATEID4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestStateId4Res {
    Ok(TestStateId4ResOk),
    Err(Stat4),
}

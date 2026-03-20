#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::Sequence4Args;
use crate::nfsv4::ops::Sequence4Res;
use crate::nfsv4::ops::layoutcommit::NewOffset4;
use crate::nfsv4::ops::layoutcommit::NewSize4;
use crate::nfsv4::ops::layoutcommit::NewTime4;
use crate::nfsv4::ops::layoutget::LayoutGet4Res;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.47.1: ssa_digest_input4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsaDigestInput4 {
    pub sdi_seqargs: Sequence4Args,
}

/// RFC8881 Section 18.47.1: SET_SSV4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSsv4Args {
    pub ssa_ssv: Vec<u8>,
    pub ssa_digest: Vec<u8>,
}

/// RFC8881 Section 18.47.2: ssr_digest_input4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsrDigestInput4 {
    pub sdi_seqres: Sequence4Res,
}

/// RFC8881 Section 18.47.2: SET_SSV4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSsv4ResOk {
    pub ssr_digest: Vec<u8>,
}

/// RFC8881 Section 18.47.2: SET_SSV4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetSsv4Res {
    Ok(SetSsv4ResOk),
    Err(Stat4),
}

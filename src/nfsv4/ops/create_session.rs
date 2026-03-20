#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::CallbackSecParms4;
use crate::nfsv4::types::ClientId4;
use crate::nfsv4::types::Count4;
use crate::nfsv4::types::SequenceId4;
use crate::nfsv4::types::SessionId4;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.36.1: channel_attrs4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelAttrs4 {
    pub ca_headerpadsize: Count4,
    pub ca_maxrequestsize: Count4,
    pub ca_maxresponsesize: Count4,
    pub ca_maxresponsesize_cached: Count4,
    pub ca_maxoperations: Count4,
    pub ca_maxrequests: Count4,
    pub ca_rdma_ird: Vec<u32>, // <1>
}

/// RFC8881 Section 18.36.1: CREATE_SESSION4 flags
pub const CREATE_SESSION4_FLAG_PERSIST: u32 = 0x00000001;
pub const CREATE_SESSION4_FLAG_CONN_BACK_CHAN: u32 = 0x00000002;
pub const CREATE_SESSION4_FLAG_CONN_RDMA: u32 = 0x00000004;

/// RFC8881 Section 18.36.1: CREATE_SESSION4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSession4Args {
    pub csa_clientid: ClientId4,
    pub csa_sequence: SequenceId4,
    pub csa_flags: u32,
    pub csa_fore_chan_attrs: ChannelAttrs4,
    pub csa_back_chan_attrs: ChannelAttrs4,
    pub csa_cb_program: u32,
    pub csa_sec_parms: Vec<CallbackSecParms4>,
}

/// RFC8881 Section 18.36.2: CREATE_SESSION4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSession4ResOk {
    pub csr_sessionid: SessionId4,
    pub csr_sequence: SequenceId4,
    pub csr_flags: u32,
    pub csr_fore_chan_attrs: ChannelAttrs4,
    pub csr_back_chan_attrs: ChannelAttrs4,
}

/// RFC8881 Section 18.36.2: CREATE_SESSION4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateSession4Res {
    Ok(CreateSession4ResOk),
    Err(Stat4),
}

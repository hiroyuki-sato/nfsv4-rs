#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC8881 Section 18.34.1: channel_dir_from_client4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChannelDirFromClient4 {
    Cdfc4Fore = 0x1,
    Cdfc4Back = 0x2,
    Cdfc4ForeOrBoth = 0x3,
    Cdfc4BackOrBoth = 0x7,
}

/// RFC8881 Section 18.34.1: BIND_CONN_TO_SESSION4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindConnToSession4Args {
    pub bctsa_sessid: SessionId4,
    pub bctsa_dir: ChannelDirFromClient4,
    pub bctsa_use_conn_in_rdma_mode: bool,
}

/// RFC8881 Section 18.34.2: channel_dir_from_server4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChannelDirFromServer4 {
    Cdfs4Fore = 0x1,
    Cdfs4Back = 0x2,
    Cdfs4Both = 0x3,
}

/// RFC8881 Section 18.34.2: BIND_CONN_TO_SESSION4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindConnToSession4ResOk {
    pub bctsr_sessid: SessionId4,
    pub bctsr_dir: ChannelDirFromServer4,
    pub bctsr_use_conn_in_rdma_mode: bool,
}

/// RFC8881 Section 18.34.2: BIND_CONN_TO_SESSION4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindConnToSession4Res {
    Ok(BindConnToSession4ResOk),
    Err(Stat4),
}

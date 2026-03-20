#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::GssHandle4;
use crate::nfsv4::types::Bitmap4;
use crate::nfsv4::types::ClientId4;
use crate::nfsv4::types::ClientOwner4;
use crate::nfsv4::types::NfsImplId4;
use crate::nfsv4::types::SecOid4;
use crate::nfsv4::types::SequenceId4;
use crate::nfsv4::types::ServerOwner4;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.35.1: EXCHANGE_ID4 flags
pub const EXCHGID4_FLAG_SUPP_MOVED_REFER: u32 = 0x00000001;
pub const EXCHGID4_FLAG_SUPP_MOVED_MIGR: u32 = 0x00000002;

pub const EXCHGID4_FLAG_BIND_PRINC_STATEID: u32 = 0x00000100;

pub const EXCHGID4_FLAG_USE_NON_PNFS: u32 = 0x00010000;
pub const EXCHGID4_FLAG_USE_PNFS_MDS: u32 = 0x00020000;
pub const EXCHGID4_FLAG_USE_PNFS_DS: u32 = 0x00040000;

pub const EXCHGID4_FLAG_MASK_PNFS: u32 = 0x00070000;

pub const EXCHGID4_FLAG_UPD_CONFIRMED_REC_A: u32 = 0x40000000;
pub const EXCHGID4_FLAG_CONFIRMED_R: u32 = 0x80000000;

/// RFC8881 Section 18.35.1: state_protect_ops4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateProtectOps4 {
    pub spo_must_enforce: Bitmap4,
    pub spo_must_allow: Bitmap4,
}

/// RFC8881 Section 18.35.1: ssv_sp_parms4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsvSpParms4 {
    pub ssp_ops: StateProtectOps4,
    pub ssp_hash_algs: Vec<SecOid4>,
    pub ssp_encr_algs: Vec<SecOid4>,
    pub ssp_window: u32,
    pub ssp_num_gss_handles: u32,
}

/// RFC8881 Section 18.35.1: state_protect_how4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StateProtectHow4 {
    Sp4None = 0,
    Sp4MachCred = 1,
    Sp4Ssv = 2,
}

/// RFC8881 Section 18.35.1: state_protect4_a
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateProtect4A {
    None,
    MachCred(StateProtectOps4),
    Ssv(SsvSpParms4),
}

/// RFC8881 Section 18.35.1: EXCHANGE_ID4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeId4Args {
    pub eia_clientowner: ClientOwner4,
    pub eia_flags: u32,
    pub eia_state_protect: StateProtect4A,
    pub eia_client_impl_id: Vec<NfsImplId4>, // <1>
}

/// RFC8881 Section 18.35.2: ssv_prot_info4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsvProtInfo4 {
    pub spi_ops: StateProtectOps4,
    pub spi_hash_alg: u32,
    pub spi_encr_alg: u32,
    pub spi_ssv_len: u32,
    pub spi_window: u32,
    pub spi_handles: Vec<GssHandle4>,
}

/// RFC8881 Section 18.35.2: state_protect4_r
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateProtect4R {
    None,
    MachCred(StateProtectOps4),
    Ssv(SsvProtInfo4),
}

/// RFC8881 Section 18.35.2: EXCHANGE_ID4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeId4ResOk {
    pub eir_clientid: ClientId4,
    pub eir_sequenceid: SequenceId4,
    pub eir_flags: u32,
    pub eir_state_protect: StateProtect4R,
    pub eir_server_owner: ServerOwner4,
    pub eir_server_scope: Vec<u8>,           // <NFS4_OPAQUE_LIMIT>
    pub eir_server_impl_id: Vec<NfsImplId4>, // <1>
}

/// RFC8881 Section 18.35.2: EXCHANGE_ID4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeId4Res {
    Ok(ExchangeId4ResOk),
    Err(Stat4),
}

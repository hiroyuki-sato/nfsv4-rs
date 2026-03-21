#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::XdrError;
use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::RpcGssSvcT;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.33.1: gsshandle4_t
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GssHandle4(Vec<u8>);

impl GssHandle4 {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self(r.read_opaque()?))
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_opaque(&self.0)?;
        Ok(())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// RFC8881 Section 18.33.1: gss_cb_handles4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GssCbHandles4 {
    /// RFC 2203
    pub gcbp_service: RpcGssSvcT,
    pub gcbp_handle_from_server: GssHandle4,
    pub gcbp_handle_from_client: GssHandle4,
}

/// TODO use OncRpcXs
/// RFC8881 Section 18.33.1: callback_sec_parms4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallbackSecParms4 {
    AuthNone,
    //AuthSys(AuthSysParms),
    //RpcSecGss(GssCbHandles4),
}

/// RFC8881 Section 18.33.1: BACKCHANNEL_CTL4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackchannelCtl4Args {
    pub bca_cb_program: u32,
    pub bca_sec_parms: Vec<CallbackSecParms4>,
}

/// RFC8881 Section 18.33.2: BACKCHANNEL_CTL4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackchannelCtl4Res {
    pub bcr_status: Stat4,
}

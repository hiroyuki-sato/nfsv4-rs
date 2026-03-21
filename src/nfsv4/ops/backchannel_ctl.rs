#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use onc_rpc_rs::auth::AuthFlavor;
use onc_rpc_rs::auth::AuthSysParams;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::RpcGssSvcT;
use crate::nfsv4::types::Stat4;
use crate::xdr_ext::{read_array_nfs, write_array_nfs};

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

impl GssCbHandles4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            gcbp_service: RpcGssSvcT::try_from(r.read_i32()?)?,
            gcbp_handle_from_server: GssHandle4::decode(r)?,
            gcbp_handle_from_client: GssHandle4::decode(r)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.gcbp_service as i32)?;
        self.gcbp_handle_from_server.encode(w)?;
        self.gcbp_handle_from_client.encode(w)?;
        Ok(())
    }
}

/// RFC8881 Section 18.33.1: callback_sec_parms4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallbackSecParms4 {
    AuthNone,
    AuthSys(AuthSysParams),
    RpcSecGss(GssCbHandles4),
}

impl CallbackSecParms4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let flavor = AuthFlavor::try_from(r.read_i32()?)?;
        match flavor {
            AuthFlavor::None => Ok(Self::AuthNone),
            AuthFlavor::Sys => Ok(Self::AuthSys(AuthSysParams::decode(r)?)),
            AuthFlavor::RpcSecGss => Ok(Self::RpcSecGss(GssCbHandles4::decode(r)?)),
            _ => Err(Nfsv4Error::UnsupportedAuthFlavor(flavor)),
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::AuthNone => w.write_i32(AuthFlavor::None as i32)?,
            Self::AuthSys(params) => {
                w.write_i32(AuthFlavor::Sys as i32)?;
                params.encode(w)?;
            }
            Self::RpcSecGss(handles) => {
                w.write_i32(AuthFlavor::RpcSecGss as i32)?;
                handles.encode(w)?;
            }
        }
        Ok(())
    }
}

/// RFC8881 Section 18.33.1: BACKCHANNEL_CTL4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackchannelCtl4Args {
    pub bca_cb_program: u32,
    pub bca_sec_parms: Vec<CallbackSecParms4>,
}

impl BackchannelCtl4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            bca_cb_program: r.read_u32()?,
            bca_sec_parms: read_array_nfs(r, CallbackSecParms4::decode)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u32(self.bca_cb_program)?;
        write_array_nfs(w, &self.bca_sec_parms, |w, item| item.encode(w))?;
        Ok(())
    }
}

/// RFC8881 Section 18.33.2: BACKCHANNEL_CTL4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackchannelCtl4Res {
    pub bcr_status: Stat4,
}

impl BackchannelCtl4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            bcr_status: Stat4::try_from(r.read_i32()?)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.bcr_status as i32)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_auth_sys() -> AuthSysParams {
        AuthSysParams {
            stamp: 12345,
            machinename: "test-host".to_string(),
            uid: 1000,
            gid: 100,
            gids: vec![100, 200, 300],
        }
    }

    #[test]
    fn test_gsshandle4_encode_decode() {
        let original = GssHandle4::new(vec![0x01, 0x02, 0x03, 0x04]);

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GssHandle4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_gsshandle4_empty_encode_decode() {
        let original = GssHandle4::new(Vec::new());

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GssHandle4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_gsshandle4_as_bytes() {
        let original = GssHandle4::new(vec![0xaa, 0xbb, 0xcc]);
        assert_eq!(original.as_bytes(), &[0xaa, 0xbb, 0xcc]);
    }

    #[test]
    fn test_gsscbhandles4_encode_decode() {
        let original = GssCbHandles4 {
            gcbp_service: RpcGssSvcT::Integrity,
            gcbp_handle_from_server: GssHandle4::new(vec![0x10, 0x20]),
            gcbp_handle_from_client: GssHandle4::new(vec![0x30, 0x40, 0x50]),
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GssCbHandles4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_callbacksecparms4_authnone_encode_decode() {
        let original = CallbackSecParms4::AuthNone;

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = CallbackSecParms4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_callbacksecparms4_authsys_encode_decode() {
        let original = CallbackSecParms4::AuthSys(sample_auth_sys());

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = CallbackSecParms4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_callbacksecparms4_rpcsecgss_encode_decode() {
        let original = CallbackSecParms4::RpcSecGss(GssCbHandles4 {
            gcbp_service: RpcGssSvcT::Privacy,
            gcbp_handle_from_server: GssHandle4::new(vec![0x01, 0x02]),
            gcbp_handle_from_client: GssHandle4::new(vec![0x03, 0x04, 0x05]),
        });

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = CallbackSecParms4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_backchannelctl4args_encode_decode() {
        let original = BackchannelCtl4Args {
            bca_cb_program: 0x4000_0001,
            bca_sec_parms: vec![
                CallbackSecParms4::AuthNone,
                CallbackSecParms4::AuthSys(sample_auth_sys()),
                CallbackSecParms4::RpcSecGss(GssCbHandles4 {
                    gcbp_service: RpcGssSvcT::None,
                    gcbp_handle_from_server: GssHandle4::new(vec![0xaa]),
                    gcbp_handle_from_client: GssHandle4::new(vec![0xbb, 0xcc]),
                }),
            ],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = BackchannelCtl4Args::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_backchannelctl4args_empty_sec_parms() {
        let original = BackchannelCtl4Args {
            bca_cb_program: 123,
            bca_sec_parms: Vec::new(),
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = BackchannelCtl4Args::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_backchannelctl4res_encode_decode_ok() {
        let original = BackchannelCtl4Res {
            bcr_status: Stat4::Ok,
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = BackchannelCtl4Res::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_backchannelctl4res_encode_decode_error() {
        let original = BackchannelCtl4Res {
            bcr_status: Stat4::Access,
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = BackchannelCtl4Res::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }
}

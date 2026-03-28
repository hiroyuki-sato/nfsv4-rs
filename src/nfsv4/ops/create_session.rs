#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::CallbackSecParms4;
use crate::nfsv4::types::ClientId4;
use crate::nfsv4::types::Count4;
use crate::nfsv4::types::SESSIONID_SIZE;
use crate::nfsv4::types::SequenceId4;
use crate::nfsv4::types::SessionId4;
use crate::nfsv4::types::Stat4;
use crate::xdr_ext::{read_array_nfs, write_array_nfs};

const DEFAULT_CSA_HEADER_PAD_SIZE: u32 = 0;
const DEFAULT_CSA_MAX_REQUEST_SIZE: u32 = 1049620u32; // 1 MiB + NFS+RPC header
const DEFAULT_CSA_MAX_RESPONSE_SIZE: u32 = 1049480u32; // 1 MiB + NFS+RPC header - 512 (worst-case header size)
const DEFAULT_CSA_MAX_RESPONSE_SIZE_CACHED: u32 = 3428u32; // 4096 (typical page size) - 512 (worst-case header size) 
const DEFAULT_CSA_MAX_OPERATIONS: u32 = 8;
const DEFAULT_CSA_MAX_REQUESTS: u32 = 64;

const DEFAULT_CB_HEADER_PAD_SIZE: u32 = 0;
const DEFAULT_CB_MAX_REQUEST_SIZE: u32 = 4096u32;
const DEFAULT_CB_MAX_RESPONSE_SIZE: u32 = 4096u32;
const DEFAULT_CB_MAX_RESPONSE_SIZE_CACHED: u32 = 0u32; // 4096 (typical page size) - 512 (worst-case header size) 
const DEFAULT_CB_MAX_OPERATIONS: u32 = 2;
const DEFAULT_CB_MAX_REQUESTS: u32 = 1;

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

impl ChannelAttrs4 {
    pub fn default_force() -> Self {
        Self {
            ca_headerpadsize: DEFAULT_CSA_HEADER_PAD_SIZE,
            ca_maxrequestsize: DEFAULT_CSA_MAX_REQUEST_SIZE,
            ca_maxresponsesize: DEFAULT_CSA_MAX_RESPONSE_SIZE,
            ca_maxresponsesize_cached: DEFAULT_CSA_MAX_RESPONSE_SIZE_CACHED,
            ca_maxoperations: DEFAULT_CSA_MAX_OPERATIONS,
            ca_maxrequests: DEFAULT_CSA_MAX_REQUESTS,
            ca_rdma_ird: vec![],
        }
    }
    pub fn default_back() -> Self {
        Self {
            ca_headerpadsize: DEFAULT_CB_HEADER_PAD_SIZE,
            ca_maxrequestsize: DEFAULT_CB_MAX_REQUEST_SIZE,
            ca_maxresponsesize: DEFAULT_CB_MAX_RESPONSE_SIZE,
            ca_maxresponsesize_cached: DEFAULT_CB_MAX_RESPONSE_SIZE_CACHED,
            ca_maxoperations: DEFAULT_CB_MAX_OPERATIONS,
            ca_maxrequests: DEFAULT_CB_MAX_REQUESTS,
            ca_rdma_ird: vec![],
        }
    }

    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            ca_headerpadsize: r.read_u32()?,
            ca_maxrequestsize: r.read_u32()?,
            ca_maxresponsesize: r.read_u32()?,
            ca_maxresponsesize_cached: r.read_u32()?,
            ca_maxoperations: r.read_u32()?,
            ca_maxrequests: r.read_u32()?,
            ca_rdma_ird: read_array_nfs(r, |r| Ok(r.read_u32()?))?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u32(self.ca_headerpadsize)?;
        w.write_u32(self.ca_maxrequestsize)?;
        w.write_u32(self.ca_maxresponsesize)?;
        w.write_u32(self.ca_maxresponsesize_cached)?;
        w.write_u32(self.ca_maxoperations)?;
        w.write_u32(self.ca_maxrequests)?;
        write_array_nfs(w, &self.ca_rdma_ird, |w, item| {
            w.write_u32(*item)?;
            Ok(())
        })?;
        Ok(())
    }
}

/// RFC8881 Section 18.36.1: CREATE_SESSION4 flags
pub const CREATE_SESSION4_FLAG_PERSIST: u32 = 0x00000001;
pub const CREATE_SESSION4_FLAG_CONN_BACK_CHAN: u32 = 0x00000002;
pub const CREATE_SESSION4_FLAG_CONN_RDMA: u32 = 0x00000004;

const DEFAULT_CSA_CB_PRGRAM: u32 = 0x40000000;

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

impl CreateSession4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            csa_clientid: r.read_u64()?,
            csa_sequence: r.read_u32()?,
            csa_flags: r.read_u32()?,
            csa_fore_chan_attrs: ChannelAttrs4::decode(r)?,
            csa_back_chan_attrs: ChannelAttrs4::decode(r)?,
            csa_cb_program: r.read_u32()?,
            csa_sec_parms: read_array_nfs(r, CallbackSecParms4::decode)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.csa_clientid)?;
        w.write_u32(self.csa_sequence)?;
        w.write_u32(self.csa_flags)?;
        self.csa_fore_chan_attrs.encode(w)?;
        self.csa_back_chan_attrs.encode(w)?;
        w.write_u32(self.csa_cb_program)?;
        write_array_nfs(w, &self.csa_sec_parms, |w, v| v.encode(w))?;
        Ok(())
    }
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

impl CreateSession4ResOk {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let csr_sessionid: SessionId4 = r
            .read_fixed_opaque(SESSIONID_SIZE)?
            .try_into()
            .map_err(|_| Nfsv4Error::InvalidData("expected sessionid4".into()))?;

        Ok(Self {
            csr_sessionid,
            csr_sequence: r.read_u32()?,
            csr_flags: r.read_u32()?,
            csr_fore_chan_attrs: ChannelAttrs4::decode(r)?,
            csr_back_chan_attrs: ChannelAttrs4::decode(r)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_fixed_opaque(&self.csr_sessionid)?;
        w.write_u32(self.csr_sequence)?;
        w.write_u32(self.csr_flags)?;
        self.csr_fore_chan_attrs.encode(w)?;
        self.csr_back_chan_attrs.encode(w)?;
        Ok(())
    }
}

/// RFC8881 Section 18.36.2: CREATE_SESSION4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateSession4Res {
    Ok(CreateSession4ResOk),
    Err(Stat4),
}

impl CreateSession4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let status = Stat4::try_from(r.read_i32()?)?;
        match status {
            Stat4::Ok => Ok(Self::Ok(CreateSession4ResOk::decode(r)?)),
            err => Ok(Self::Err(err)),
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::Ok(ok) => {
                w.write_i32(Stat4::Ok as i32)?;
                ok.encode(w)?;
            }
            Self::Err(err) => w.write_i32(*err as i32)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nfsv4::ops::{CallbackSecParms4, GssCbHandles4, GssHandle4, RpcGssSvcT};
    use onc_rpc_rs::auth::AuthSysParams;

    fn sample_channel_attrs() -> ChannelAttrs4 {
        ChannelAttrs4 {
            ca_headerpadsize: 1,
            ca_maxrequestsize: 2,
            ca_maxresponsesize: 3,
            ca_maxresponsesize_cached: 4,
            ca_maxoperations: 5,
            ca_maxrequests: 6,
            ca_rdma_ird: vec![7],
        }
    }

    fn sample_callback_sec_parms() -> Vec<CallbackSecParms4> {
        vec![
            CallbackSecParms4::AuthNone,
            CallbackSecParms4::AuthSys(AuthSysParams {
                stamp: 123,
                machinename: "client.example".to_string(),
                uid: 1000,
                gid: 100,
                gids: vec![100, 200],
            }),
            CallbackSecParms4::RpcSecGss(GssCbHandles4 {
                gcbp_service: RpcGssSvcT::Integrity,
                gcbp_handle_from_server: GssHandle4::new(vec![0xaa, 0xbb]),
                gcbp_handle_from_client: GssHandle4::new(vec![0xcc, 0xdd, 0xee]),
            }),
        ]
    }

    #[test]
    fn test_channelattrs4_encode_decode() {
        let original = sample_channel_attrs();

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ChannelAttrs4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_channelattrs4_empty_rdma_ird() {
        let original = ChannelAttrs4 {
            ca_headerpadsize: 10,
            ca_maxrequestsize: 20,
            ca_maxresponsesize: 30,
            ca_maxresponsesize_cached: 40,
            ca_maxoperations: 50,
            ca_maxrequests: 60,
            ca_rdma_ird: vec![],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ChannelAttrs4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_createsession4args_encode_decode() {
        let original = CreateSession4Args {
            csa_clientid: 0x1122_3344_5566_7788,
            csa_sequence: 42,
            csa_flags: CREATE_SESSION4_FLAG_PERSIST | CREATE_SESSION4_FLAG_CONN_BACK_CHAN,
            csa_fore_chan_attrs: sample_channel_attrs(),
            csa_back_chan_attrs: ChannelAttrs4 {
                ca_headerpadsize: 11,
                ca_maxrequestsize: 22,
                ca_maxresponsesize: 33,
                ca_maxresponsesize_cached: 44,
                ca_maxoperations: 55,
                ca_maxrequests: 66,
                ca_rdma_ird: vec![77],
            },
            csa_cb_program: 0x4000_0001,
            csa_sec_parms: sample_callback_sec_parms(),
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = CreateSession4Args::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_createsession4resok_encode_decode() {
        let original = CreateSession4ResOk {
            csr_sessionid: [0x11; SESSIONID_SIZE],
            csr_sequence: 99,
            csr_flags: CREATE_SESSION4_FLAG_CONN_RDMA,
            csr_fore_chan_attrs: sample_channel_attrs(),
            csr_back_chan_attrs: ChannelAttrs4 {
                ca_headerpadsize: 101,
                ca_maxrequestsize: 102,
                ca_maxresponsesize: 103,
                ca_maxresponsesize_cached: 104,
                ca_maxoperations: 105,
                ca_maxrequests: 106,
                ca_rdma_ird: vec![107],
            },
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = CreateSession4ResOk::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_createsession4res_ok_encode_decode() {
        let original = CreateSession4Res::Ok(CreateSession4ResOk {
            csr_sessionid: [0x22; SESSIONID_SIZE],
            csr_sequence: 7,
            csr_flags: CREATE_SESSION4_FLAG_PERSIST,
            csr_fore_chan_attrs: sample_channel_attrs(),
            csr_back_chan_attrs: ChannelAttrs4 {
                ca_headerpadsize: 8,
                ca_maxrequestsize: 9,
                ca_maxresponsesize: 10,
                ca_maxresponsesize_cached: 11,
                ca_maxoperations: 12,
                ca_maxrequests: 13,
                ca_rdma_ird: vec![14],
            },
        });

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = CreateSession4Res::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_createsession4res_err_encode_decode() {
        let original = CreateSession4Res::Err(Stat4::Access);

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = CreateSession4Res::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_channelattrs4_default_force_values() {
        let attrs = ChannelAttrs4::default_force();

        assert_eq!(attrs.ca_headerpadsize, DEFAULT_CSA_HEADER_PAD_SIZE);
        assert_eq!(attrs.ca_maxrequestsize, DEFAULT_CSA_MAX_REQUEST_SIZE);
        assert_eq!(attrs.ca_maxresponsesize, DEFAULT_CSA_MAX_RESPONSE_SIZE);
        assert_eq!(
            attrs.ca_maxresponsesize_cached,
            DEFAULT_CSA_MAX_RESPONSE_SIZE_CACHED
        );
        assert_eq!(attrs.ca_maxoperations, DEFAULT_CSA_MAX_OPERATIONS);
        assert_eq!(attrs.ca_maxrequests, DEFAULT_CSA_MAX_REQUESTS);
        assert!(attrs.ca_rdma_ird.is_empty());
    }

    #[test]
    fn test_channelattrs4_default_back_values() {
        let attrs = ChannelAttrs4::default_back();

        assert_eq!(attrs.ca_headerpadsize, DEFAULT_CB_HEADER_PAD_SIZE);
        assert_eq!(attrs.ca_maxrequestsize, DEFAULT_CB_MAX_REQUEST_SIZE);
        assert_eq!(attrs.ca_maxresponsesize, DEFAULT_CB_MAX_RESPONSE_SIZE);
        assert_eq!(
            attrs.ca_maxresponsesize_cached,
            DEFAULT_CB_MAX_RESPONSE_SIZE_CACHED
        );
        assert_eq!(attrs.ca_maxoperations, DEFAULT_CB_MAX_OPERATIONS);
        assert_eq!(attrs.ca_maxrequests, DEFAULT_CB_MAX_REQUESTS);
        assert!(attrs.ca_rdma_ird.is_empty());
    }

    #[test]
    fn test_channelattrs4_force_and_back_are_different() {
        let fore = ChannelAttrs4::default_force();
        let back = ChannelAttrs4::default_back();

        assert!(
            fore.ca_headerpadsize != back.ca_headerpadsize
                || fore.ca_maxrequestsize != back.ca_maxrequestsize
                || fore.ca_maxresponsesize != back.ca_maxresponsesize
                || fore.ca_maxresponsesize_cached != back.ca_maxresponsesize_cached
                || fore.ca_maxoperations != back.ca_maxoperations
                || fore.ca_maxrequests != back.ca_maxrequests
        );
    }

    //
    #[test]
    fn test_create_session4_args() {
        let args = CreateSession4Args {
            csa_clientid: 0x1234_5678_9abc_def0,
            csa_sequence: 123,
            csa_flags: CREATE_SESSION4_FLAG_CONN_BACK_CHAN,
            csa_fore_chan_attrs: ChannelAttrs4::default_force(),
            csa_back_chan_attrs: ChannelAttrs4::default_back(),
            csa_cb_program: DEFAULT_CSA_CB_PRGRAM,
            csa_sec_parms: vec![CallbackSecParms4::auth_sys_user_1000("test-client")],
        };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = CreateSession4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }
}

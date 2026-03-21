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
use crate::xdr_ext::{read_array_nfs, write_array_nfs};

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

impl StateProtectOps4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            spo_must_enforce: Bitmap4::decode(r)?,
            spo_must_allow: Bitmap4::decode(r)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.spo_must_enforce.encode(w)?;
        self.spo_must_allow.encode(w)?;
        Ok(())
    }
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

impl SsvSpParms4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            ssp_ops: StateProtectOps4::decode(r)?,
            ssp_hash_algs: read_array_nfs(r, SecOid4::decode)?,
            ssp_encr_algs: read_array_nfs(r, SecOid4::decode)?,
            ssp_window: r.read_u32()?,
            ssp_num_gss_handles: r.read_u32()?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.ssp_ops.encode(w)?;
        write_array_nfs(w, &self.ssp_hash_algs, |w, alg| alg.encode(w))?;
        write_array_nfs(w, &self.ssp_encr_algs, |w, alg| alg.encode(w))?;
        w.write_u32(self.ssp_window)?;
        w.write_u32(self.ssp_num_gss_handles)?;
        Ok(())
    }
}

/// RFC8881 Section 18.35.1: state_protect_how4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StateProtectHow4 {
    Sp4None = 0,
    Sp4MachCred = 1,
    Sp4Ssv = 2,
}

impl StateProtectHow4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        match r.read_i32()? {
            0 => Ok(Self::Sp4None),
            1 => Ok(Self::Sp4MachCred),
            2 => Ok(Self::Sp4Ssv),
            v => Err(Nfsv4Error::InvalidStateProtectHow(v)),
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(*self as i32)?;
        Ok(())
    }
}

/// RFC8881 Section 18.35.1: state_protect4_a
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateProtect4A {
    None,
    MachCred(StateProtectOps4),
    Ssv(SsvSpParms4),
}

impl StateProtect4A {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let how = StateProtectHow4::decode(r)?;
        match how {
            StateProtectHow4::Sp4None => Ok(Self::None),
            StateProtectHow4::Sp4MachCred => Ok(Self::MachCred(StateProtectOps4::decode(r)?)),
            StateProtectHow4::Sp4Ssv => Ok(Self::Ssv(SsvSpParms4::decode(r)?)),
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::None => StateProtectHow4::Sp4None.encode(w)?,
            Self::MachCred(ops) => {
                StateProtectHow4::Sp4MachCred.encode(w)?;
                ops.encode(w)?;
            }
            Self::Ssv(ssp) => {
                StateProtectHow4::Sp4Ssv.encode(w)?;
                ssp.encode(w)?;
            }
        }
        Ok(())
    }
}

/// RFC8881 Section 18.35.1: EXCHANGE_ID4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeId4Args {
    pub eia_clientowner: ClientOwner4,
    pub eia_flags: u32,
    pub eia_state_protect: StateProtect4A,
    pub eia_client_impl_id: Vec<NfsImplId4>, // <1>
}

impl ExchangeId4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            eia_clientowner: ClientOwner4::decode(r)?,
            eia_flags: r.read_u32()?,
            eia_state_protect: StateProtect4A::decode(r)?,
            eia_client_impl_id: read_array_nfs(r, NfsImplId4::decode)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.eia_clientowner.encode(w)?;
        w.write_u32(self.eia_flags)?;
        self.eia_state_protect.encode(w)?;
        write_array_nfs(w, &self.eia_client_impl_id, |w, id| id.encode(w))?;
        Ok(())
    }
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

impl SsvProtInfo4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            spi_ops: StateProtectOps4::decode(r)?,
            spi_hash_alg: r.read_u32()?,
            spi_encr_alg: r.read_u32()?,
            spi_ssv_len: r.read_u32()?,
            spi_window: r.read_u32()?,
            spi_handles: read_array_nfs(r, GssHandle4::decode)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.spi_ops.encode(w)?;
        w.write_u32(self.spi_hash_alg)?;
        w.write_u32(self.spi_encr_alg)?;
        w.write_u32(self.spi_ssv_len)?;
        w.write_u32(self.spi_window)?;
        write_array_nfs(w, &self.spi_handles, |w, h| h.encode(w))?;
        Ok(())
    }
}

/// RFC8881 Section 18.35.2: state_protect4_r
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateProtect4R {
    None,
    MachCred(StateProtectOps4),
    Ssv(SsvProtInfo4),
}

impl StateProtect4R {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let how = StateProtectHow4::decode(r)?;
        match how {
            StateProtectHow4::Sp4None => Ok(Self::None),
            StateProtectHow4::Sp4MachCred => Ok(Self::MachCred(StateProtectOps4::decode(r)?)),
            StateProtectHow4::Sp4Ssv => Ok(Self::Ssv(SsvProtInfo4::decode(r)?)),
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::None => StateProtectHow4::Sp4None.encode(w)?,
            Self::MachCred(ops) => {
                StateProtectHow4::Sp4MachCred.encode(w)?;
                ops.encode(w)?;
            }
            Self::Ssv(prot) => {
                StateProtectHow4::Sp4Ssv.encode(w)?;
                prot.encode(w)?;
            }
        }
        Ok(())
    }
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

impl ExchangeId4ResOk {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            eir_clientid: r.read_u64()?,
            eir_sequenceid: r.read_u32()?,
            eir_flags: r.read_u32()?,
            eir_state_protect: StateProtect4R::decode(r)?,
            eir_server_owner: ServerOwner4::decode(r)?,
            eir_server_scope: r.read_opaque()?,
            eir_server_impl_id: read_array_nfs(r, NfsImplId4::decode)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.eir_clientid)?;
        w.write_u32(self.eir_sequenceid)?;
        w.write_u32(self.eir_flags)?;
        self.eir_state_protect.encode(w)?;
        self.eir_server_owner.encode(w)?;
        w.write_opaque(&self.eir_server_scope)?;
        write_array_nfs(w, &self.eir_server_impl_id, |w, id| id.encode(w))?;
        Ok(())
    }
}

/// RFC8881 Section 18.35.2: EXCHANGE_ID4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeId4Res {
    Ok(ExchangeId4ResOk),
    Err(Stat4),
}

impl ExchangeId4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let status = Stat4::try_from(r.read_i32()?)?;
        match status {
            Stat4::Ok => Ok(Self::Ok(ExchangeId4ResOk::decode(r)?)),
            _ => Ok(Self::Err(status)),
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::Ok(resok) => {
                w.write_i32(Stat4::Ok as i32)?; // status
                resok.encode(w)?;
            }
            Self::Err(stat) => {
                w.write_i32(*stat as i32)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nfsv4::types::NfsTime4;

    fn sample_bitmap(bits: &[u32]) -> Bitmap4 {
        let mut bm = Bitmap4::new();
        for &bit in bits {
            bm.insert(bit);
        }
        bm
    }

    fn sample_client_owner() -> ClientOwner4 {
        ClientOwner4 {
            co_verifier: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
            co_ownerid: vec![0xaa, 0xbb, 0xcc],
        }
    }

    fn sample_server_owner() -> ServerOwner4 {
        ServerOwner4 {
            so_minor_id: 0x1122_3344_5566_7788,
            so_major_id: vec![0xde, 0xad, 0xbe, 0xef],
        }
    }

    fn sample_impl_id() -> NfsImplId4 {
        NfsImplId4 {
            nii_domain: "example.com".to_string(),
            nii_name: "nfsv4-rs".to_string(),
            nii_date: crate::nfsv4::types::NfsTime4 {
                seconds: 1_700_000_000,
                nseconds: 123_456_789,
            },
        }
    }

    #[test]
    fn test_stateprotectops4_encode_decode() {
        let v = StateProtectOps4 {
            spo_must_enforce: sample_bitmap(&[0, 31, 32]),
            spo_must_allow: sample_bitmap(&[1, 2, 63]),
        };

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = StateProtectOps4::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_ssvspparms4_encode_decode() {
        let v = SsvSpParms4 {
            ssp_ops: StateProtectOps4 {
                spo_must_enforce: sample_bitmap(&[0, 2]),
                spo_must_allow: sample_bitmap(&[1, 3]),
            },
            ssp_hash_algs: vec![
                SecOid4::new(vec![0x2a, 0x86, 0x48]),
                SecOid4::new(vec![0x2b, 0x06, 0x01]),
            ],
            ssp_encr_algs: vec![SecOid4::new(vec![0x60, 0x86, 0x48])],
            ssp_window: 16,
            ssp_num_gss_handles: 2,
        };

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SsvSpParms4::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_stateprotecthow4_encode_decode() {
        for v in [
            StateProtectHow4::Sp4None,
            StateProtectHow4::Sp4MachCred,
            StateProtectHow4::Sp4Ssv,
        ] {
            let mut w = XdrWriter::new();
            v.encode(&mut w).unwrap();

            let mut r = XdrReader::new(w.as_bytes());
            let decoded = StateProtectHow4::decode(&mut r).unwrap();

            assert_eq!(v, decoded);
        }
    }

    #[test]
    fn test_stateprotecthow4_decode_invalid() {
        let mut w = XdrWriter::new();
        w.write_i32(99).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let err = StateProtectHow4::decode(&mut r).unwrap_err();

        assert!(matches!(err, Nfsv4Error::InvalidStateProtectHow(99)));
    }

    #[test]
    fn test_stateprotect4a_none_encode_decode() {
        let v = StateProtect4A::None;

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = StateProtect4A::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_stateprotect4a_machcred_encode_decode() {
        let v = StateProtect4A::MachCred(StateProtectOps4 {
            spo_must_enforce: sample_bitmap(&[0, 4]),
            spo_must_allow: sample_bitmap(&[1, 5]),
        });

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = StateProtect4A::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_stateprotect4a_ssv_encode_decode() {
        let v = StateProtect4A::Ssv(SsvSpParms4 {
            ssp_ops: StateProtectOps4 {
                spo_must_enforce: sample_bitmap(&[0]),
                spo_must_allow: sample_bitmap(&[1]),
            },
            ssp_hash_algs: vec![SecOid4::new(vec![0x01, 0x02])],
            ssp_encr_algs: vec![SecOid4::new(vec![0x03, 0x04])],
            ssp_window: 8,
            ssp_num_gss_handles: 1,
        });

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = StateProtect4A::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_exchangeid4args_encode_decode() {
        let v = ExchangeId4Args {
            eia_clientowner: sample_client_owner(),
            eia_flags: EXCHGID4_FLAG_SUPP_MOVED_REFER | EXCHGID4_FLAG_USE_NON_PNFS,
            eia_state_protect: StateProtect4A::MachCred(StateProtectOps4 {
                spo_must_enforce: sample_bitmap(&[0, 1]),
                spo_must_allow: sample_bitmap(&[2, 3]),
            }),
            eia_client_impl_id: vec![sample_impl_id()],
        };

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ExchangeId4Args::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_ssvprotinfo4_encode_decode() {
        let v = SsvProtInfo4 {
            spi_ops: StateProtectOps4 {
                spo_must_enforce: sample_bitmap(&[0]),
                spo_must_allow: sample_bitmap(&[1]),
            },
            spi_hash_alg: 10,
            spi_encr_alg: 20,
            spi_ssv_len: 32,
            spi_window: 4,
            spi_handles: vec![
                GssHandle4::new(vec![0xaa, 0xbb]),
                GssHandle4::new(vec![0xcc, 0xdd, 0xee]),
            ],
        };

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SsvProtInfo4::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_stateprotect4r_none_encode_decode() {
        let v = StateProtect4R::None;

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = StateProtect4R::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_stateprotect4r_machcred_encode_decode() {
        let v = StateProtect4R::MachCred(StateProtectOps4 {
            spo_must_enforce: sample_bitmap(&[7]),
            spo_must_allow: sample_bitmap(&[8]),
        });

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = StateProtect4R::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_stateprotect4r_ssv_encode_decode() {
        let v = StateProtect4R::Ssv(SsvProtInfo4 {
            spi_ops: StateProtectOps4 {
                spo_must_enforce: sample_bitmap(&[0, 9]),
                spo_must_allow: sample_bitmap(&[1, 10]),
            },
            spi_hash_alg: 1,
            spi_encr_alg: 2,
            spi_ssv_len: 64,
            spi_window: 16,
            spi_handles: vec![GssHandle4::new(vec![0x10, 0x20])],
        });

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = StateProtect4R::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_exchangeid4resok_encode_decode() {
        let v = ExchangeId4ResOk {
            eir_clientid: 0x1122_3344_5566_7788,
            eir_sequenceid: 42,
            eir_flags: EXCHGID4_FLAG_CONFIRMED_R,
            eir_state_protect: StateProtect4R::MachCred(StateProtectOps4 {
                spo_must_enforce: sample_bitmap(&[0]),
                spo_must_allow: sample_bitmap(&[1]),
            }),
            eir_server_owner: sample_server_owner(),
            eir_server_scope: vec![0xfa, 0xfb, 0xfc],
            eir_server_impl_id: vec![sample_impl_id()],
        };

        let mut w = XdrWriter::new();
        v.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ExchangeId4ResOk::decode(&mut r).unwrap();

        assert_eq!(v, decoded);
    }

    #[test]
    fn test_exchangeid4res_ok_encode_decode() {
        let res = ExchangeId4Res::Ok(ExchangeId4ResOk {
            eir_clientid: 0x1122_3344_5566_7788,
            eir_sequenceid: 42,
            eir_flags: EXCHGID4_FLAG_CONFIRMED_R,
            eir_state_protect: StateProtect4R::None,
            eir_server_owner: ServerOwner4 {
                so_minor_id: 0x8877_6655_4433_2211,
                so_major_id: vec![0xaa, 0xbb, 0xcc],
            },
            eir_server_scope: vec![0x10, 0x20, 0x30],
            eir_server_impl_id: vec![NfsImplId4 {
                nii_domain: "example.com".to_string(),
                nii_name: "nfsv4-rs".to_string(),
                nii_date: NfsTime4 {
                    seconds: 1_700_000_000,
                    nseconds: 123_456_789,
                },
            }],
        });

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ExchangeId4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_exchangeid4res_err_encode_decode() {
        let res = ExchangeId4Res::Err(Stat4::Access);

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ExchangeId4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }
}

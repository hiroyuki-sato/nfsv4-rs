#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::Component4;
use crate::nfsv4::types::Qop4;
use crate::nfsv4::types::SecOid4;
use crate::nfsv4::types::Stat4;
use crate::xdr_ext::{read_array_nfs, write_array_nfs};

/// RFC7531: SECINFO4args
///
/// Arguments for the SECINFO operation.
/// CURRENT_FH must refer to a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecInfo4Args {
    /// Name of the object whose security information is requested.
    pub name: Component4,
}

impl SecInfo4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            name: r.read_string()?,
        })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_string(&self.name)?;
        Ok(())
    }
}

/// RFC2203: rpc_gss_svc_t
///
/// RPCSEC_GSS service type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RpcGssSvcT {
    /// No security service.
    None = 1,

    /// Integrity protection service.
    Integrity = 2,

    /// Privacy protection service.
    Privacy = 3,
}

impl TryFrom<i32> for RpcGssSvcT {
    type Error = Nfsv4Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::None),
            2 => Ok(Self::Integrity),
            3 => Ok(Self::Privacy),
            _ => Err(Nfsv4Error::InvalidRpcGssSvcT(value)),
        }
    }
}

/// RFC7531: rpcsec_gss_info
///
/// RPCSEC_GSS security information returned by SECINFO.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcSecGssInfo {
    /// Security mechanism OID.
    pub oid: SecOid4,

    /// Quality of protection.
    pub qop: Qop4,

    /// RPCSEC_GSS service type.
    pub service: RpcGssSvcT,
}

impl RpcSecGssInfo {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            oid: SecOid4::decode(r)?,
            qop: r.read_u32()?,
            service: RpcGssSvcT::try_from(r.read_i32()?)?,
        })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.oid.encode(w)?;
        w.write_u32(self.qop)?;
        w.write_i32(self.service as i32)?;
        Ok(())
    }
}

/// RPCSEC_GSS flavor value defined by RFC2203.
pub const RPCSEC_GSS: u32 = 6;

/// RFC7531: secinfo4
///
/// Security flavor information returned by SECINFO.
///
/// The discriminator is the RPC authentication flavor number.
/// For flavors other than RPCSEC_GSS, no additional data is returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecInfo4 {
    /// RPCSEC_GSS flavor with additional GSS information.
    RpcSecGss(RpcSecGssInfo),

    /// Other security flavor with no additional flavor-specific data.
    Other(u32),
}

impl SecInfo4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let flavor = r.read_u32()?;
        if flavor == RPCSEC_GSS {
            Ok(Self::RpcSecGss(RpcSecGssInfo::decode(r)?))
        } else {
            Ok(Self::Other(flavor))
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::RpcSecGss(info) => {
                w.write_u32(RPCSEC_GSS)?;
                info.encode(w)?;
            }
            Self::Other(flavor) => {
                w.write_u32(*flavor)?;
            }
        }
        Ok(())
    }
}

/// RFC7531: SECINFO4resok
///
/// Successful result payload of the SECINFO operation.
///
/// In XDR this is:
/// `typedef secinfo4 SECINFO4resok<>;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecInfo4ResOk(Vec<SecInfo4>);

impl SecInfo4ResOk {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self(read_array_nfs(r, SecInfo4::decode)?))
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        write_array_nfs(w, &self.0, |w, info| info.encode(w))?;
        Ok(())
    }
    pub fn infos(&self) -> &[SecInfo4] {
        &self.0
    }
}

/// RFC7531: SECINFO4res
///
/// Result of the SECINFO operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecInfo4Res {
    /// Operation succeeded.
    Ok(SecInfo4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

impl SecInfo4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let status = Stat4::try_from(r.read_i32()?)?;
        match status {
            Stat4::Ok => Ok(Self::Ok(SecInfo4ResOk::decode(r)?)),
            err => Ok(Self::Err(err)),
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::Ok(resok) => {
                w.write_i32(Stat4::Ok as i32)?;
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

    #[test]
    fn test_secinfo4args_encode_decode() {
        let args = SecInfo4Args {
            name: "testfile".to_string(),
        };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecInfo4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }

    #[test]
    fn test_rpcgsssvct_try_from_valid() {
        assert!(matches!(RpcGssSvcT::try_from(1).unwrap(), RpcGssSvcT::None));
        assert!(matches!(
            RpcGssSvcT::try_from(2).unwrap(),
            RpcGssSvcT::Integrity
        ));
        assert!(matches!(
            RpcGssSvcT::try_from(3).unwrap(),
            RpcGssSvcT::Privacy
        ));
    }

    #[test]
    fn test_rpcgsssvct_try_from_invalid() {
        let err = RpcGssSvcT::try_from(99).unwrap_err();
        assert!(matches!(err, Nfsv4Error::InvalidRpcGssSvcT(99)));
    }

    #[test]
    fn test_rpcsecgssinfo_encode_decode() {
        let info = RpcSecGssInfo {
            oid: SecOid4::new(vec![0x2a, 0x86, 0x48]),
            qop: 42,
            service: RpcGssSvcT::Integrity,
        };

        let mut w = XdrWriter::new();
        info.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = RpcSecGssInfo::decode(&mut r).unwrap();

        assert_eq!(info, decoded);
    }

    #[test]
    fn test_secinfo4_rpcsecgss_encode_decode() {
        let info = SecInfo4::RpcSecGss(RpcSecGssInfo {
            oid: SecOid4::new(vec![0x2a, 0x86, 0x48, 0x86]),
            qop: 7,
            service: RpcGssSvcT::Privacy,
        });

        let mut w = XdrWriter::new();
        info.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecInfo4::decode(&mut r).unwrap();

        assert_eq!(info, decoded);
    }

    #[test]
    fn test_secinfo4_other_encode_decode() {
        let info = SecInfo4::Other(1);

        let mut w = XdrWriter::new();
        info.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecInfo4::decode(&mut r).unwrap();

        assert_eq!(info, decoded);
    }

    #[test]
    fn test_secinfo4resok_encode_decode() {
        let resok = SecInfo4ResOk(vec![
            SecInfo4::Other(0),
            SecInfo4::RpcSecGss(RpcSecGssInfo {
                oid: SecOid4::new(vec![0x01, 0x02, 0x03]),
                qop: 99,
                service: RpcGssSvcT::None,
            }),
        ]);

        let mut w = XdrWriter::new();
        resok.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecInfo4ResOk::decode(&mut r).unwrap();

        assert_eq!(resok, decoded);
    }

    #[test]
    fn test_secinfo4resok_infos() {
        let resok = SecInfo4ResOk(vec![SecInfo4::Other(0), SecInfo4::Other(RPCSEC_GSS)]);

        assert_eq!(resok.infos().len(), 2);
        assert!(matches!(resok.infos()[0], SecInfo4::Other(0)));
        assert!(matches!(resok.infos()[1], SecInfo4::Other(RPCSEC_GSS)));
    }

    #[test]
    fn test_secinfo4res_ok_encode_decode() {
        let res = SecInfo4Res::Ok(SecInfo4ResOk(vec![
            SecInfo4::RpcSecGss(RpcSecGssInfo {
                oid: SecOid4::new(vec![0xaa, 0xbb]),
                qop: 5,
                service: RpcGssSvcT::Integrity,
            }),
            SecInfo4::Other(1),
        ]));

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecInfo4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_secinfo4res_err_encode_decode() {
        let res = SecInfo4Res::Err(Stat4::Access);

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecInfo4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }
}

#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: SECINFO4args
///
/// Arguments for the SECINFO operation.
/// CURRENT_FH must refer to a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecInfo4Args {
    /// Name of the object whose security information is requested.
    pub name: Component4,
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

/// RFC7531: SECINFO4resok
///
/// Successful result payload of the SECINFO operation.
///
/// In XDR this is:
/// `typedef secinfo4 SECINFO4resok<>;`
pub type SecInfo4ResOk = Vec<SecInfo4>;

/// RFC7531: SECINFO4res
///
/// Result of the SECINFO operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecInfo4Res {
    /// Operation succeeded.
    Ok(SecInfo4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

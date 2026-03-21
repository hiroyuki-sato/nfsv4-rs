use xdr_rs::error::XdrError;

use onc_rpc_rs::auth::AuthFlavor;
use onc_rpc_rs::error::RpcError;

#[derive(Debug)]
pub enum Nfsv4Error {
    Xdr(XdrError),
    RpcError(RpcError),
    InvalidOp(u32),
    InvalidEnumValue(i32),
    InvalidUtf8,
    InvalidLength {
        field: &'static str,
        len: usize,
        max: usize,
    },
    UnexpectedEof,
    TrailingBytes {
        remaining: usize,
    },
    InvalidNfsOpnum4(i32),
    InvalidData(String),
    InvalidNfsStatus(i32),
    InvalidNfsFiletype(i32),
    InvalidTimeHow4(i32),
    InvalidLayoutType(i32),
    InvalidLayoutIOMode(i32),
    InvalidStateProtectHow(i32),
    InvalidRpcGssSvcT(i32),
    UnsupportedAuthFlavor(AuthFlavor),
}

impl From<XdrError> for Nfsv4Error {
    fn from(err: XdrError) -> Self {
        Nfsv4Error::Xdr(err)
    }
}

impl From<RpcError> for Nfsv4Error {
    fn from(err: RpcError) -> Self {
        Nfsv4Error::RpcError(err)
    }
}

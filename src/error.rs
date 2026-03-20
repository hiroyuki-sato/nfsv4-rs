use xdr_rs::error::XdrError;

#[derive(Debug)]
pub enum Nfsv4Error {
    Xdr(XdrError),
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
}

impl From<XdrError> for Nfsv4Error {
    fn from(err: XdrError) -> Self {
        Nfsv4Error::Xdr(err)
    }
}

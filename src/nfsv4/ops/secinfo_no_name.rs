#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::SecInfo4Res;

/// RFC8881 Section 18.45.1: secinfo_style4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SecInfoStyle4 {
    SecInfoStyle4CurrentFh = 0,
    SecInfoStyle4Parent = 1,
}

/// RFC8881 Section 18.45.1: SECINFO_NO_NAME4args
///
/// CURRENT_FH: object or child directory
pub type SecInfoNoName4Args = SecInfoStyle4;

/// RFC8881 Section 18.45.2: SECINFO_NO_NAME4res
///
/// CURRENT_FH: consumed if status is NFS4_OK
pub type SecInfoNoName4Res = SecInfo4Res;

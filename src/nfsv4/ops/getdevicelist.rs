#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::Count4;
use crate::nfsv4::types::DeviceId4;
use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::NfsCookie4;
use crate::nfsv4::types::Stat4;
use crate::nfsv4::types::Verifier4;

/// RFC8881 Section 18.41.1: GETDEVICELIST4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceList4Args {
    /// CURRENT_FH: object belonging to the file system
    pub gdla_layout_type: LayoutType4,

    /// number of deviceIDs to return
    pub gdla_maxdevices: Count4,

    pub gdla_cookie: NfsCookie4,
    pub gdla_cookieverf: Verifier4,
}

/// RFC8881 Section 18.41.2: GETDEVICELIST4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceList4ResOk {
    pub gdlr_cookie: NfsCookie4,
    pub gdlr_cookieverf: Verifier4,
    pub gdlr_deviceid_list: Vec<DeviceId4>,
    pub gdlr_eof: bool,
}

/// RFC8881 Section 18.41.2: GETDEVICELIST4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetDeviceList4Res {
    Ok(GetDeviceList4ResOk),
    Err(Stat4),
}

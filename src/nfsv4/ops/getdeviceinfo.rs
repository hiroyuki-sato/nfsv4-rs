#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::Bitmap4;
use crate::nfsv4::types::Count4;
use crate::nfsv4::types::DeviceAddr4;
use crate::nfsv4::types::DeviceId4;
use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.40.1: GETDEVICEINFO4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceInfo4Args {
    pub gdia_device_id: DeviceId4,
    pub gdia_layout_type: LayoutType4,
    pub gdia_maxcount: Count4,
    pub gdia_notify_types: Bitmap4,
}

/// RFC8881 Section 18.40.2: GETDEVICEINFO4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceInfo4ResOk {
    pub gdir_device_addr: DeviceAddr4,
    pub gdir_notification: Bitmap4,
}

/// RFC8881 Section 18.40.2: GETDEVICEINFO4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetDeviceInfo4Res {
    Ok(GetDeviceInfo4ResOk),
    TooSmall(Count4),
    Err(Stat4),
}

#![allow(dead_code)]

use super::*;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::nfsv4::types::Nfsv4Error;

/// RFC8881 Section 3.3.15: device_addr4
///
/// Describes the address information for a device in pNFS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceAddr4 {
    /// Layout type associated with this device
    pub da_layout_type: LayoutType4,

    /// Opaque layout-specific address body
    pub da_addr_body: Vec<u8>,
}

impl DeviceAddr4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let layout_type = LayoutType4::try_from(r.read_i32()?)?;
        let addr_body = r.read_opaque()?;
        Ok(Self {
            da_layout_type: layout_type,
            da_addr_body: addr_body,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.da_layout_type as i32)?;
        w.write_opaque(&self.da_addr_body)?;
        Ok(())
    }
}

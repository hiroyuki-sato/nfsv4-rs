#![allow(dead_code)]

use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.16: layout_content4
///
/// Layout content returned for pNFS operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutContent4 {
    /// Layout type
    pub loc_type: LayoutType4,

    /// Opaque layout-specific content
    pub loc_body: Vec<u8>,
}

impl LayoutContent4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let loc_type = LayoutType4::try_from(r.read_i32()?)?;
        let loc_body = r.read_opaque()?;
        Ok(Self { loc_type, loc_body })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.loc_type as i32)?;
        w.write_opaque(&self.loc_body)?;
        Ok(())
    }
}

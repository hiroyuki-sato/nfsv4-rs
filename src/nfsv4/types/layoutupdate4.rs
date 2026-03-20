#![allow(dead_code)]

use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.18: layoutupdate4
///
/// Layout update information returned to the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutUpdate4 {
    /// Layout type
    pub lou_type: LayoutType4,

    /// Opaque layout-specific update body
    pub lou_body: Vec<u8>,
}

impl LayoutUpdate4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let lou_type = LayoutType4::try_from(r.read_i32()?)?;
        let lou_body = r.read_opaque()?;
        Ok(Self { lou_type, lou_body })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.lou_type as i32)?;
        w.write_opaque(&self.lou_body)?;
        Ok(())
    }
}

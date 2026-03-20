#![allow(dead_code)]

use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.19: layouthint4
///
/// Hint provided by the client about desired layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutHint4 {
    /// Layout type
    pub loh_type: LayoutType4,

    /// Opaque layout-specific hint body
    pub loh_body: Vec<u8>,
}

impl LayoutHint4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let loh_type = LayoutType4::try_from(r.read_i32()?)?;
        let loh_body = r.read_opaque()?;
        Ok(Self { loh_type, loh_body })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.loh_type as i32)?;
        w.write_opaque(&self.loh_body)?;
        Ok(())
    }
}

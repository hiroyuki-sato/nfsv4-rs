#![allow(dead_code)]

use crate::nfsv4::types::LayoutContent4;
use crate::nfsv4::types::LayoutIOMode4;
use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Length4;
use crate::nfsv4::types::Nfsv4Error;
use crate::nfsv4::types::Offset4;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.17: layout4
///
/// Describes a layout segment for pNFS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout4 {
    /// Offset within the file
    pub lo_offset: Offset4,

    /// Length of the layout segment
    pub lo_length: Length4,

    /// I/O mode for the layout
    pub lo_iomode: LayoutIOMode4,

    /// Layout-specific content
    pub lo_content: LayoutContent4,
}

impl Layout4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let lo_offset = r.read_u64()?;
        let lo_length = r.read_u64()?;
        let lo_iomode = LayoutIOMode4::try_from(r.read_i32()?)?;
        let lo_content = LayoutContent4::decode(r)?;
        Ok(Self {
            lo_offset,
            lo_length,
            lo_iomode,
            lo_content,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.lo_offset)?;
        w.write_u64(self.lo_length)?;
        w.write_i32(self.lo_iomode as i32)?;
        self.lo_content.encode(w)?;
        Ok(())
    }
}

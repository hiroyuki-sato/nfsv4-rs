#![allow(dead_code)]

use crate::nfsv4::types::Bitmap4;
use crate::nfsv4::types::ClientId4;
use crate::nfsv4::types::LayoutType4;
use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.22 Table 2: threshold4_*
///
/// Bit numbers used in `threshold_item4.thi_hintset`.

/// If a file's length is less than this value, it is RECOMMENDED
/// that the client read via the MDS rather than a storage device.
pub const THRESHOLD4_READ_SIZE: u32 = 0;

/// If a file's length is less than this value, it is RECOMMENDED
/// that the client write via the MDS rather than a storage device.
pub const THRESHOLD4_WRITE_SIZE: u32 = 1;

/// For read I/O sizes below this threshold, it is RECOMMENDED
/// to read data through the MDS.
pub const THRESHOLD4_READ_IOSIZE: u32 = 2;

/// For write I/O sizes below this threshold, it is RECOMMENDED
/// to write data through the MDS.
pub const THRESHOLD4_WRITE_IOSIZE: u32 = 3;

/// RFC8881 Section 3.3.22: threshold_item4
///
/// Threshold hint item used for layout-specific threshold information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThresholdItem4 {
    /// Layout type
    pub thi_layout_type: LayoutType4,

    /// Bitmap indicating which hints are present
    pub thi_hintset: Bitmap4,

    /// Opaque layout-specific hint list
    pub thi_hintlist: Vec<u8>,
}

impl ThresholdItem4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let thi_layout_type = LayoutType4::try_from(r.read_i32()?)?;
        let thi_hintset = Bitmap4::decode(r)?;
        let thi_hintlist = r.read_opaque()?;
        Ok(Self {
            thi_layout_type,
            thi_hintset,
            thi_hintlist,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.thi_layout_type as i32)?;
        self.thi_hintset.encode(w)?;
        w.write_opaque(&self.thi_hintlist)?;
        Ok(())
    }
}

#![allow(dead_code)]

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::nfsv4::types::LayoutType4;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deviceaddr4_encode_decode() {
        let original = DeviceAddr4 {
            da_layout_type: LayoutType4::NfsV4_1Files,
            da_addr_body: vec![0x01, 0x02, 0x03, 0x04],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = DeviceAddr4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_deviceaddr4_empty_body() {
        let original = DeviceAddr4 {
            da_layout_type: LayoutType4::BlockVolume,
            da_addr_body: vec![],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = DeviceAddr4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_deviceaddr4_decode_invalid_layout_type() {
        let mut w = XdrWriter::new();
        w.write_i32(99).unwrap();
        w.write_opaque(&[0xaa, 0xbb]).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let err = DeviceAddr4::decode(&mut r).unwrap_err();

        assert!(matches!(err, Nfsv4Error::InvalidLayoutType(99)));
    }

    #[test]
    fn test_deviceaddr4_decode_truncated_body() {
        let buf = [0x00, 0x00, 0x00, 0x01, 0x00, 0x00];
        let mut r = XdrReader::new(&buf);

        let err = DeviceAddr4::decode(&mut r).unwrap_err();
        assert!(matches!(err, Nfsv4Error::Xdr(_)));
    }
}

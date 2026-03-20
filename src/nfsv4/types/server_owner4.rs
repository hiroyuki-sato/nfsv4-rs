#![allow(dead_code)]

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::nfsv4::types::Nfsv4Error;

/// RFC8881 Section 2.5: server_owner4
///
/// Identifies the server instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerOwner4 {
    /// Minor identifier (usually server incarnation)
    pub so_minor_id: u64,

    /// Major identifier (opaque server identity)
    pub so_major_id: Vec<u8>,
}

impl ServerOwner4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let so_minor_id = r.read_u64()?;
        let so_major_id = r.read_opaque()?; // <NFS4_OPAQUE_LIMIT>
        Ok(Self {
            so_minor_id,
            so_major_id,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.so_minor_id)?;
        w.write_opaque(&self.so_major_id)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serverowner4_encode_decode() {
        let original = ServerOwner4 {
            so_minor_id: 0x1122_3344_5566_7788,
            so_major_id: vec![0xaa, 0xbb, 0xcc, 0xdd],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ServerOwner4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_serverowner4_encode_decode_empty_major_id() {
        let original = ServerOwner4 {
            so_minor_id: 0,
            so_major_id: Vec::new(),
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ServerOwner4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_serverowner4_decode_truncated_minor_id() {
        let buf = [0x11, 0x22, 0x33, 0x44]; // less than u64

        let mut r = XdrReader::new(&buf);
        let err = ServerOwner4::decode(&mut r).unwrap_err();

        assert!(matches!(err, Nfsv4Error::Xdr(_)));
    }
}

#![allow(dead_code)]

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::nfsv4::types::Nfsv4Error;
use crate::nfsv4::types::VERIFIER_SIZE;
use crate::nfsv4::types::Verifier4;

/// RFC8881 Section 18.35.x: client_owner4
///
/// Identifies the client instance for EXCHANGE_ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientOwner4 {
    /// Client verifier
    pub co_verifier: Verifier4,

    /// Opaque client owner identifier
    pub co_ownerid: Vec<u8>,
}

impl ClientOwner4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let co_verifier: Verifier4 = r
            .read_fixed_opaque(VERIFIER_SIZE)?
            .try_into()
            .map_err(|_| Nfsv4Error::InvalidData("expected verifier4".into()))?;
        let co_ownerid = r.read_opaque()?; // <NFS4_OPAQUE_LIMIT>
        Ok(Self {
            co_verifier,
            co_ownerid,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_fixed_opaque(&self.co_verifier)?;
        w.write_opaque(&self.co_ownerid)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clientowner4_encode_decode() {
        let original = ClientOwner4 {
            co_verifier: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
            co_ownerid: vec![0xaa, 0xbb, 0xcc, 0xdd],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ClientOwner4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_clientowner4_encode_decode_empty_ownerid() {
        let original = ClientOwner4 {
            co_verifier: [0; VERIFIER_SIZE],
            co_ownerid: Vec::new(),
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ClientOwner4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_clientowner4_decode_truncated_verifier() {
        let buf = [0x11, 0x22, 0x33, 0x44]; // less than VERIFIER_SIZE

        let mut r = XdrReader::new(&buf);
        let err = ClientOwner4::decode(&mut r).unwrap_err();

        assert!(matches!(err, Nfsv4Error::Xdr(_)));
    }
}

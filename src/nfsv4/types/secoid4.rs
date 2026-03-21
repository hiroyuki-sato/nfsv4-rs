#![allow(dead_code)]

use xdr_rs::XdrError;
use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::nfsv4::types::Nfsv4Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecOid4(Vec<u8>);

impl SecOid4 {
    pub fn new(v: Vec<u8>) -> Self {
        Self(v)
    }

    pub fn decode_xdr(r: &mut XdrReader) -> Result<Self, XdrError> {
        Ok(Self(r.read_opaque()?))
    }

    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self::decode_xdr(r)?)
    }

    pub fn encode_xdr(&self, w: &mut XdrWriter) -> Result<(), XdrError> {
        w.write_opaque(&self.0)?;
        Ok(())
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.encode_xdr(w)?;
        Ok(())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secoid4_encode_decode() {
        let original = SecOid4(vec![0x01, 0x02, 0x03, 0x04]);

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecOid4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_secoid4_empty() {
        let original = SecOid4(vec![]);

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecOid4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_secoid4_as_bytes() {
        let data = vec![0xaa, 0xbb, 0xcc];
        let oid = SecOid4(data.clone());

        assert_eq!(oid.as_bytes(), &data[..]);
    }

    #[test]
    fn test_secoid4_decode_xdr() {
        let original = SecOid4(vec![0x01, 0x02, 0x03, 0x04]);

        let mut w = XdrWriter::new();
        original.encode_xdr(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecOid4::decode_xdr(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_secoid4_encode_xdr() {
        let original = SecOid4(vec![0xaa, 0xbb, 0xcc]);

        let mut w = XdrWriter::new();
        original.encode_xdr(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecOid4::decode_xdr(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_secoid4_encode_decode_xdr_empty() {
        let original = SecOid4(vec![]);

        let mut w = XdrWriter::new();
        original.encode_xdr(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SecOid4::decode_xdr(&mut r).unwrap();

        assert_eq!(original, decoded);
    }
}

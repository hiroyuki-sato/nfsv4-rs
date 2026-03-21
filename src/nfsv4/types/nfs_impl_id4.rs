#![allow(dead_code)]

use crate::nfsv4::types::NfsTime4;
use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::XdrError;
use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.21: nfs_impl_id4
///
/// Identifies the implementation (vendor/domain/name/date).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NfsImplId4 {
    /// Implementation domain (case-insensitive)
    pub nii_domain: String,

    /// Implementation name (case-sensitive)
    pub nii_name: String,

    /// Implementation date
    pub nii_date: NfsTime4,
}

impl NfsImplId4 {
    pub fn decode_xdr(r: &mut XdrReader) -> Result<Self, XdrError> {
        let nii_domain = r.read_string()?;
        let nii_name = r.read_string()?;
        let nii_date = NfsTime4::decode_xdr(r)?;
        Ok(Self {
            nii_domain,
            nii_name,
            nii_date,
        })
    }

    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self::decode_xdr(r)?)
    }

    pub fn encode_xdr(&self, w: &mut XdrWriter) -> Result<(), XdrError> {
        w.write_string(&self.nii_domain)?;
        w.write_string(&self.nii_name)?;
        self.nii_date.encode_xdr(w)?;
        Ok(())
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.encode_xdr(w)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfsimplid4_encode_decode() {
        let original = NfsImplId4 {
            nii_domain: "example.com".to_string(),
            nii_name: "nfsv4-rs".to_string(),
            nii_date: NfsTime4 {
                seconds: 1_700_000_000,
                nseconds: 123_456_789,
            },
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = NfsImplId4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_nfsimplid4_encode_decode_xdr() {
        let original = NfsImplId4 {
            nii_domain: "example.org".to_string(),
            nii_name: "server-impl".to_string(),
            nii_date: NfsTime4 {
                seconds: 42,
                nseconds: 999,
            },
        };

        let mut w = XdrWriter::new();
        original.encode_xdr(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = NfsImplId4::decode_xdr(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_nfsimplid4_empty_strings() {
        let original = NfsImplId4 {
            nii_domain: String::new(),
            nii_name: String::new(),
            nii_date: NfsTime4 {
                seconds: 0,
                nseconds: 0,
            },
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = NfsImplId4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_nfsimplid4_decode_xdr_truncated() {
        let buf = [0x00, 0x00, 0x00, 0x03, b'a', b'b'];
        let mut r = XdrReader::new(&buf);

        let err = NfsImplId4::decode_xdr(&mut r).unwrap_err();
        let _ = err;
    }

    #[test]
    fn test_nfsimplid4_decode_truncated() {
        let buf = [0x00, 0x00, 0x00, 0x03, b'a', b'b'];
        let mut r = XdrReader::new(&buf);

        let err = NfsImplId4::decode(&mut r).unwrap_err();
        assert!(matches!(err, Nfsv4Error::Xdr(_)));
    }
}

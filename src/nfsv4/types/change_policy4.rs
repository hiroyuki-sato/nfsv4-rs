#![allow(dead_code)]

use super::*;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::nfsv4::types::Nfsv4Error;

/// RFC8881: change_policy4
///
/// Used to describe change attribute policy (major/minor versioning).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangePolicy4 {
    /// Major change policy value
    pub cp_major: u64,

    /// Minor change policy value
    pub cp_minor: u64,
}

impl ChangePolicy4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let cp_major = r.read_u64()?;
        let cp_minor = r.read_u64()?;
        Ok(Self { cp_major, cp_minor })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.cp_major)?;
        w.write_u64(self.cp_minor)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_changepolicy4_encode_decode() {
        let original = ChangePolicy4 {
            cp_major: 0x1122_3344_5566_7788,
            cp_minor: 0x8877_6655_4433_2211,
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ChangePolicy4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_changepolicy4_zero_values() {
        let original = ChangePolicy4 {
            cp_major: 0,
            cp_minor: 0,
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ChangePolicy4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_changepolicy4_max_values() {
        let original = ChangePolicy4 {
            cp_major: u64::MAX,
            cp_minor: u64::MAX,
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ChangePolicy4::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_changepolicy4_decode_truncated() {
        let buf = [0x11, 0x22, 0x33, 0x44]; // less than 16 bytes
        let mut r = XdrReader::new(&buf);

        let err = ChangePolicy4::decode(&mut r).unwrap_err();
        assert!(matches!(err, Nfsv4Error::Xdr(_)));
    }
}

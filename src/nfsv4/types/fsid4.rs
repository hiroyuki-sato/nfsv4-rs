#![allow(dead_code)]

use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC7531: fsid4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fsid4 {
    pub major: u64,
    pub minor: u64,
}

impl Fsid4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let major = r.read_u64()?;
        let minor = r.read_u64()?;
        Ok(Self { major, minor })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.major)?;
        w.write_u64(self.minor)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fsid4_encode_decode() {
        let fsid = Fsid4 {
            major: 123456789,
            minor: 987654321,
        };

        let mut w = XdrWriter::new();
        fsid.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Fsid4::decode(&mut r).unwrap();

        assert_eq!(fsid, decoded);
    }

    #[test]
    fn test_fsid4_zero() {
        let fsid = Fsid4 { major: 0, minor: 0 };

        let mut w = XdrWriter::new();
        fsid.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Fsid4::decode(&mut r).unwrap();

        assert_eq!(fsid, decoded);
    }

    #[test]
    fn test_fsid4_max_values() {
        let fsid = Fsid4 {
            major: u64::MAX,
            minor: u64::MAX,
        };

        let mut w = XdrWriter::new();
        fsid.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Fsid4::decode(&mut r).unwrap();

        assert_eq!(fsid, decoded);
    }
}

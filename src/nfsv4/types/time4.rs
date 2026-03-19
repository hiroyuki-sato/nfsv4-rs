#![allow(dead_code)]

use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC7531: time4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time4 {
    pub seconds: i64,
    pub nseconds: u32,
}

impl Time4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let seconds = r.read_i64()?;
        let nseconds = r.read_u32()?;
        Ok(Self { seconds, nseconds })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i64(self.seconds)?;
        w.write_u32(self.nseconds)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfstime4_encode_decode() {
        let time = Time4 {
            seconds: 123456789,
            nseconds: 987654321,
        };

        let mut w = XdrWriter::new();
        time.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Time4::decode(&mut r).unwrap();

        assert_eq!(time, decoded);
    }

    #[test]
    fn test_nfstime4_zero() {
        let time = Time4 {
            seconds: 0,
            nseconds: 0,
        };

        let mut w = XdrWriter::new();
        time.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Time4::decode(&mut r).unwrap();

        assert_eq!(time, decoded);
    }

    #[test]
    fn test_nfstime4_negative_seconds() {
        let time = Time4 {
            seconds: -1,
            nseconds: 500_000_000,
        };

        let mut w = XdrWriter::new();
        time.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Time4::decode(&mut r).unwrap();

        assert_eq!(time, decoded);
    }
}

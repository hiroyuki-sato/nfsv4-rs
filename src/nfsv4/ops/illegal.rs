#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::Stat4;

/// RFC7531: ILLEGAL4res
///
/// Result returned when the server encounters an undefined
/// or unsupported operation in a COMPOUND request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Illegal4Res {
    /// NFS operation status.
    pub status: Stat4,
}

impl Illegal4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let status = Stat4::try_from(r.read_i32()?)?;
        Ok(Self { status })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.status as i32)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xdr_rs::reader::XdrReader;
    use xdr_rs::writer::XdrWriter;

    #[test]
    fn illegal4res_encode_writes_status_as_i32() {
        let res = Illegal4Res {
            status: Stat4::OpIllegal,
        };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let raw = r.read_i32().unwrap();
        assert_eq!(raw, Stat4::OpIllegal as i32);
    }

    #[test]
    fn illegal4res_decode_reads_status() {
        let mut w = XdrWriter::new();
        w.write_i32(Stat4::OpIllegal as i32).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let decoded = Illegal4Res::decode(&mut r).unwrap();

        assert_eq!(
            decoded,
            Illegal4Res {
                status: Stat4::OpIllegal,
            }
        );
    }

    #[test]
    fn illegal4res_roundtrip() {
        let original = Illegal4Res {
            status: Stat4::OpIllegal,
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let decoded = Illegal4Res::decode(&mut r).unwrap();

        assert_eq!(decoded, original);
    }

    #[test]
    fn illegal4res_decode_invalid_status_returns_error() {
        let mut w = XdrWriter::new();
        w.write_i32(123456789).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let err = Illegal4Res::decode(&mut r).unwrap_err();

        match err {
            Nfsv4Error::InvalidNfsStatus(123456789) => {}
            other => panic!("unexpected error: {:?}", other),
        }
    }
}

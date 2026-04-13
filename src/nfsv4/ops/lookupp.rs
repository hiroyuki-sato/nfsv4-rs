#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LookupP4Args;

impl LookupP4Args {
    pub fn decode(_r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {})
    }
    pub fn encode(&self, _w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        Ok(())
    }
}

/// RFC7531: LOOKUPP4res
///
/// Result of the LOOKUPP operation (lookup parent directory).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupP4Res {
    /// NFS operation status.
    pub status: Stat4,
}

impl LookupP4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            status: Stat4::try_from(r.read_i32()?)?,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.status as i32)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookupp4args_encode_decode() {
        let original = LookupP4Args;

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = LookupP4Args::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_lookupp4args_encode_writes_no_bytes() {
        let args = LookupP4Args;

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        assert!(w.as_bytes().is_empty());
    }

    #[test]
    fn test_lookupp4args_decode_from_empty() {
        let mut r = XdrReader::new(&[]);
        let decoded = LookupP4Args::decode(&mut r).unwrap();

        assert_eq!(decoded, LookupP4Args);
    }
}

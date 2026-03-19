#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: LOOKUP4args
///
/// Arguments for the LOOKUP operation.
/// CURRENT_FH must refer to a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookup4Args {
    /// Name of the object to look up.
    pub objname: Component4,
}

impl Lookup4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let objname = r.read_string()?;
        Ok(Self { objname })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_string(&self.objname)?;
        Ok(())
    }
}

/// RFC7531: LOOKUP4res
///
/// Result of the LOOKUP operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookup4Res {
    /// NFS operation status.
    pub status: Stat4,
}

impl Lookup4Res {
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
    fn test_lookup4args_encode_decode() {
        let args = Lookup4Args {
            objname: "testfile".to_string(),
        };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Lookup4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }

    #[test]
    fn test_lookup4args_empty_string() {
        let args = Lookup4Args {
            objname: String::new(),
        };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Lookup4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }
    #[test]
    fn test_lookup4res_encode_decode_ok() {
        let res = Lookup4Res { status: Stat4::Ok };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Lookup4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_lookup4res_encode_decode_error() {
        let res = Lookup4Res {
            status: Stat4::BadStateId,
        };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Lookup4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }
}

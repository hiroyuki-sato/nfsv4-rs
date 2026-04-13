#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

// 16.22.  Operation 24: PUTROOTFH - Set Root Filehandle
//
// 16.22.1.  SYNOPSIS
//
//      - -> (cfh)
//
// 16.22.2.  ARGUMENT
//
//      void;
//
// 16.22.3.  RESULT
//
//    struct PUTROOTFH4res {
//            /* CURRENT_FH: root fh */
//            stat4        status;
//    };
//
// 16.22.4.  DESCRIPTION
//
//    PUTROOTFH replaces the current filehandle with the filehandle that
//    represents the root of the server's namespace.  From this filehandle,
//    a LOOKUP operation can locate any other filehandle on the server.
//    This filehandle may be different from the public filehandle, which
//    may be associated with some other directory on the server.
//
//    See Section 15.2.4.1 for more details on the current filehandle.
//
// 16.22.5.  IMPLEMENTATION
//
//    PUTROOTFH is commonly used as the first operator in an NFS request to
//    set the context for operations that follow it.

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PutRootFh4Args;

impl PutRootFh4Args {
    pub fn decode(_r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {})
    }
    pub fn encode(&self, _w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        Ok(())
    }
}

/// RFC7531: PUTROOTFH4res
///
/// Result of the PUTROOTFH operation.
/// On success, CURRENT_FH becomes the root filehandle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutRootFh4Res {
    /// NFS operation status.
    pub status: Stat4,
}

impl PutRootFh4Res {
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
    fn test_putrootfh4res_encode_decode_roundtrip_ok() {
        let original = PutRootFh4Res { status: Stat4::Ok };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = PutRootFh4Res::decode(&mut r).unwrap();

        assert_eq!(decoded, original);
    }

    #[test]
    fn test_putrootfh4res_decode_ok() {
        let mut w = XdrWriter::new();
        w.write_i32(Stat4::Ok as i32).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = PutRootFh4Res::decode(&mut r).unwrap();

        assert_eq!(decoded.status, Stat4::Ok);
    }

    #[test]
    fn test_putrootfh4res_encode_ok() {
        let res = PutRootFh4Res { status: Stat4::Ok };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let raw = r.read_i32().unwrap();

        assert_eq!(raw, Stat4::Ok as i32);
    }

    #[test]
    fn test_putrootfh4res_encode_decode() {
        let res = PutRootFh4Res {
            status: Stat4::BadStateId,
        };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());

        let decoded = PutRootFh4Res::decode(&mut r).unwrap();
        assert_eq!(res, decoded);
    }
}

#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: GETFH4resok
///
/// Successful result of the GETFH operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFh4ResOk {
    /// Returned filehandle for CURRENT_FH.
    pub object: NfsFh4,
}

impl GetFh4ResOk {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let object = r.read_opaque()?;
        Ok(Self { object })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_opaque(&self.object)?;
        Ok(())
    }
}

/// RFC7531: GETFH4res
///
/// Result of the GETFH operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetFh4Res {
    /// Operation succeeded.
    Ok(GetFh4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

impl GetFh4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let status = Stat4::try_from(r.read_i32()?)?;
        match status {
            Stat4::Ok => {
                let resok = GetFh4ResOk::decode(r)?;
                Ok(Self::Ok(resok))
            }
            err => Ok(Self::Err(err)),
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::Ok(ok) => {
                w.write_i32(Stat4::Ok as i32)?;
                ok.encode(w)?;
            }
            Self::Err(err) => {
                w.write_i32(*err as i32)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getfh4resok_encode_decode() {
        let original = GetFh4ResOk {
            object: vec![0x01, 0x02, 0x03, 0x04, 0xaa, 0xbb],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetFh4ResOk::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_getfh4resok_empty_filehandle() {
        let original = GetFh4ResOk { object: vec![] };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetFh4ResOk::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_getfh4res_ok_encode_decode() {
        let original = GetFh4Res::Ok(GetFh4ResOk {
            object: vec![0x10, 0x20, 0x30, 0x40],
        });

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetFh4Res::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_getfh4res_err_encode_decode() {
        let original = GetFh4Res::Err(Stat4::Access);

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetFh4Res::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_getfh4res_decode_invalid_status() {
        let mut w = XdrWriter::new();
        w.write_i32(99999).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let err = GetFh4Res::decode(&mut r).unwrap_err();

        assert!(matches!(err, Nfsv4Error::InvalidNfsStatus(99999)));
    }
}

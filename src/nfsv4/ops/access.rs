#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: ACCESS4args
///
/// Arguments for the ACCESS operation.
///
/// The client uses this operation to check access permissions
/// for the object identified by CURRENT_FH.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Access4Args {
    /// Bitmask of requested access types.
    /// Uses ACCESS4_* constants.
    pub access: u32,
}

impl Access4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let access = r.read_u32()?;
        Ok(Self { access })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u32(self.access)?;
        Ok(())
    }
}

/// RFC7531: ACCESS4resok
///
/// Successful result of the ACCESS operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Access4ResOk {
    /// Bitmask of access types supported by the server.
    pub supported: u32,

    /// Bitmask of access types granted to the client.
    pub access: u32,
}

impl Access4ResOk {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let supported = r.read_u32()?;
        let access = r.read_u32()?;
        Ok(Self { supported, access })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u32(self.supported)?;
        w.write_u32(self.access)?;
        Ok(())
    }
}

/// RFC7531: ACCESS4res
///
/// Result of the ACCESS operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Access4Res {
    /// Operation succeeded.
    Ok(Access4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

impl Access4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let status = Stat4::decode(r)?;
        match status {
            Stat4::Ok => Ok(Self::Ok(Access4ResOk::decode(r)?)),
            err => Ok(Self::Err(err)),
        }
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::Ok(resok) => {
                w.write_i32(Stat4::Ok as i32)?;
                resok.encode(w)?;
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

    // Acess4Args
    #[test]
    fn test_access4args_encode_decode() {
        let args = Access4Args { access: 0x00000001 };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Access4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }

    #[test]
    fn test_access4args_zero() {
        let args = Access4Args { access: 0 };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Access4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }

    #[test]
    fn test_access4args_all_bits() {
        let args = Access4Args { access: 0xFFFFFFFF };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Access4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }

    // Access4ResOk
    #[test]
    fn test_access4resok_encode_decode() {
        let res = Access4ResOk {
            supported: 0x00000003,
            access: 0x00000001,
        };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Access4ResOk::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_access4resok_zero() {
        let res = Access4ResOk {
            supported: 0,
            access: 0,
        };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Access4ResOk::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_access4resok_all_bits() {
        let res = Access4ResOk {
            supported: 0xFFFFFFFF,
            access: 0xFFFFFFFF,
        };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Access4ResOk::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_access4res_ok_encode_decode() {
        let res = Access4Res::Ok(Access4ResOk {
            supported: 0x3,
            access: 0x1,
        });

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Access4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_access4res_err_encode_decode() {
        let res = Access4Res::Err(Stat4::Access);

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Access4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }
}

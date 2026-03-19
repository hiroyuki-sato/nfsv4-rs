#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: GETATTR4args
///
/// Arguments for the GETATTR operation.
/// CURRENT_FH must refer to a file or directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAttr4Args {
    /// Bitmap of requested attributes.
    pub attr_request: Bitmap4,
}

impl GetAttr4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let attr_request = Bitmap4::decode(r)?;
        Ok(Self { attr_request })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.attr_request.encode(w)?;
        Ok(())
    }
}

/// RFC7531: GETATTR4resok
///
/// Successful result of the GETATTR operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAttr4ResOk {
    /// Returned object attributes.
    pub obj_attributes: Fattr4,
}

impl GetAttr4ResOk {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let obj_attributes = Fattr4::decode(r)?;
        Ok(Self { obj_attributes })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.obj_attributes.encode(w)?;
        Ok(())
    }
}

/// RFC7531: GETATTR4res
///
/// Result of the GETATTR operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetAttr4Res {
    /// Operation succeeded.
    Ok(GetAttr4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

impl GetAttr4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let status = NfsStat4::decode(r)?;
        match status {
            NfsStat4::Ok => Ok(Self::Ok(GetAttr4ResOk::decode(r)?)),
            err => Ok(Self::Err(err)),
        }
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            Self::Ok(ok) => {
                NfsStat4::Ok.encode(w)?;
                ok.encode(w)?;
            }
            Self::Err(err) => {
                err.encode(w)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // GetAttr4Args
    #[test]
    fn test_getattr4args_encode_decode() {
        let mut bitmap = Bitmap4::new();
        bitmap.insert(0);
        bitmap.insert(31);
        bitmap.insert(32);

        let args = GetAttr4Args {
            attr_request: bitmap,
        };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetAttr4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }

    #[test]
    fn test_getattr4args_empty() {
        let args = GetAttr4Args {
            attr_request: Bitmap4::new(),
        };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetAttr4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }

    // GetAttr4ResOk
    #[test]
    fn test_getattr4resok_encode_decode() {
        let mut attrmask = Bitmap4::new();
        attrmask.insert(0);
        attrmask.insert(31);
        attrmask.insert(32);

        let res = GetAttr4ResOk {
            obj_attributes: Fattr4 {
                attrmask,
                attr_vals: vec![0xaa, 0xbb, 0xcc],
            },
        };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetAttr4ResOk::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_getattr4resok_empty() {
        let res = GetAttr4ResOk {
            obj_attributes: Fattr4 {
                attrmask: Bitmap4::new(),
                attr_vals: Vec::new(),
            },
        };

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetAttr4ResOk::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    // GetAttr4Res
    #[test]
    fn test_getattr4res_ok_encode_decode() {
        let mut attrmask = Bitmap4::new();
        attrmask.insert(0);
        attrmask.insert(31);

        let res = GetAttr4Res::Ok(GetAttr4ResOk {
            obj_attributes: Fattr4 {
                attrmask,
                attr_vals: vec![0x11, 0x22],
            },
        });

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetAttr4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_getattr4res_err_encode_decode() {
        let res = GetAttr4Res::Err(NfsStat4::BadStateId);

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = GetAttr4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }
}

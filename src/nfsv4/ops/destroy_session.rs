#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::SESSIONID_SIZE;
use crate::nfsv4::types::SessionId4;
use crate::nfsv4::types::Stat4;

/// RFC8881 Section 18.37.1: DESTROY_SESSION4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestroySession4Args {
    pub dsa_sessionid: SessionId4,
}

impl DestroySession4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let dsa_sessionid: SessionId4 = r
            .read_fixed_opaque(SESSIONID_SIZE)?
            .try_into()
            .map_err(|_| Nfsv4Error::InvalidData("expected sessionid4".into()))?;

        Ok(Self { dsa_sessionid })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_fixed_opaque(&self.dsa_sessionid)?;
        Ok(())
    }
}

/// RFC8881 Section 18.37.2: DESTROY_SESSION4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestroySession4Res {
    pub dsr_status: Stat4,
}

impl DestroySession4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let dsr_status = Stat4::try_from(r.read_i32()?)?;
        Ok(Self { dsr_status })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.dsr_status as i32)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destroysession4args_encode_decode() {
        let original = DestroySession4Args {
            dsa_sessionid: [0x11; SESSIONID_SIZE],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = DestroySession4Args::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_destroysession4args_decode_truncated() {
        let buf = [0x11, 0x22, 0x33, 0x44];
        let mut r = XdrReader::new(&buf);

        let err = DestroySession4Args::decode(&mut r).unwrap_err();
        assert!(matches!(err, Nfsv4Error::Xdr(_)));
    }

    #[test]
    fn test_destroysession4res_encode_decode_ok() {
        let original = DestroySession4Res {
            dsr_status: Stat4::Ok,
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = DestroySession4Res::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_destroysession4res_encode_decode_error() {
        let original = DestroySession4Res {
            dsr_status: Stat4::Access,
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = DestroySession4Res::decode(&mut r).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_destroysession4res_decode_invalid_status() {
        let mut w = XdrWriter::new();
        w.write_i32(99999).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let err = DestroySession4Res::decode(&mut r).unwrap_err();

        assert!(matches!(err, Nfsv4Error::InvalidNfsStatus(99999)));
    }
}

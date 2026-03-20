#![allow(dead_code)]

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::nfsv4::types::Nfsv4Error;
use crate::nfsv4::types::time_how4::TimeHow4;
use crate::nfsv4::types::time4::NfsTime4;

/// RFC7531: settime4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetTime4 {
    SetToServerTime,
    SetToClientTime(NfsTime4),
}

impl SetTime4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let how = r.read_i32()?;
        match TimeHow4::try_from(how)? {
            TimeHow4::SetToServerTime => Ok(SetTime4::SetToServerTime),
            TimeHow4::SetToClientTime => {
                let time = NfsTime4::decode(r)?;
                Ok(SetTime4::SetToClientTime(time))
            }
        }
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            SetTime4::SetToServerTime => w.write_i32(TimeHow4::SetToServerTime as i32)?,
            SetTime4::SetToClientTime(time) => {
                w.write_i32(TimeHow4::SetToClientTime as i32)?;
                time.encode(w)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_settime4_server_time_encode_decode() {
        let val = SetTime4::SetToServerTime;

        let mut w = XdrWriter::new();
        val.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SetTime4::decode(&mut r).unwrap();

        assert_eq!(val, decoded);
    }

    #[test]
    fn test_settime4_client_time_encode_decode() {
        let val = SetTime4::SetToClientTime(NfsTime4 {
            seconds: 123,
            nseconds: 456,
        });

        let mut w = XdrWriter::new();
        val.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = SetTime4::decode(&mut r).unwrap();

        assert_eq!(val, decoded);
    }

    #[test]
    fn test_settime4_invalid_discriminant() {
        let mut w = XdrWriter::new();
        w.write_i32(99).unwrap(); // invalid TimeHow4

        let mut r = XdrReader::new(w.as_bytes());
        let err = SetTime4::decode(&mut r).unwrap_err();

        assert!(matches!(err, Nfsv4Error::InvalidTimeHow4(99)));
    }
}

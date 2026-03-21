//! Helper functions for working with XDR while returning `Nfsv4Error`
//! instead of `XdrError`.
//!
//! These helpers are useful when decoding or encoding NFSv4 protocol
//! structures that may fail with either low-level XDR errors or
//! higher-level NFSv4 validation errors.

use crate::error::Nfsv4Error;
use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// Reads a variable-length XDR array using an element decoder that returns
/// `Nfsv4Error` instead of `XdrError`.
pub fn read_array_nfs<T, F>(r: &mut XdrReader, mut f: F) -> Result<Vec<T>, Nfsv4Error>
where
    F: FnMut(&mut XdrReader) -> Result<T, Nfsv4Error>,
{
    let len = r.read_u32()? as usize;
    let mut items = Vec::with_capacity(len);
    for _ in 0..len {
        items.push(f(r)?);
    }
    Ok(items)
}

/// Writes a variable-length XDR array using an element encoder that returns
/// `Nfsv4Error` instead of `XdrError`.
pub fn write_array_nfs<T, F>(w: &mut XdrWriter, items: &[T], mut f: F) -> Result<(), Nfsv4Error>
where
    F: FnMut(&mut XdrWriter, &T) -> Result<(), Nfsv4Error>,
{
    w.write_u32(items.len() as u32)?;
    for item in items {
        f(w, item)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use xdr_rs::reader::XdrReader;
    use xdr_rs::writer::XdrWriter;

    #[test]
    fn test_read_array_nfs_u32() {
        let mut w = XdrWriter::new();
        w.write_u32(3).unwrap();
        w.write_u32(10).unwrap();
        w.write_u32(20).unwrap();
        w.write_u32(30).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let values = read_array_nfs(&mut r, |r| Ok(r.read_u32()?)).unwrap();

        assert_eq!(values, vec![10, 20, 30]);
    }

    #[test]
    fn test_write_array_nfs_u32() {
        let values = vec![10u32, 20u32, 30u32];

        let mut w = XdrWriter::new();
        write_array_nfs(&mut w, &values, |w, v| {
            w.write_u32(*v)?;
            Ok(())
        })
        .unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let len = r.read_u32().unwrap();
        let v1 = r.read_u32().unwrap();
        let v2 = r.read_u32().unwrap();
        let v3 = r.read_u32().unwrap();

        assert_eq!(len, 3);
        assert_eq!(v1, 10);
        assert_eq!(v2, 20);
        assert_eq!(v3, 30);
    }

    #[test]
    fn test_read_array_nfs_propagates_nfs_error() {
        let mut w = XdrWriter::new();
        w.write_u32(2).unwrap();
        w.write_u32(1).unwrap();
        w.write_u32(999).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let err = read_array_nfs(&mut r, |r| {
            let v = r.read_u32()?;
            if v == 999 {
                return Err(Nfsv4Error::InvalidData("invalid test value".into()));
            }
            Ok(v)
        })
        .unwrap_err();

        assert!(matches!(err, Nfsv4Error::InvalidData(_)));
    }

    #[test]
    fn test_write_array_nfs_propagates_nfs_error() {
        let values = vec![1u32, 999u32];

        let mut w = XdrWriter::new();
        let err = write_array_nfs(&mut w, &values, |w, v| {
            if *v == 999 {
                return Err(Nfsv4Error::InvalidData("invalid test value".into()));
            }
            w.write_u32(*v)?;
            Ok(())
        })
        .unwrap_err();

        assert!(matches!(err, Nfsv4Error::InvalidData(_)));
    }
}

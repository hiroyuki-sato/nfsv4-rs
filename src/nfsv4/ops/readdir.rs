#![allow(dead_code)]

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: READDIR4args
///
/// Arguments for the READDIR operation.
/// CURRENT_FH must refer to a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadDir4Args {
    /// Directory cookie for continuing a readdir operation.
    pub cookie: NfsCookie4,

    /// Cookie verifier returned by a previous READDIR reply.
    pub cookieverf: Verifier4,

    /// Maximum space to use for directory entries, excluding attributes.
    pub dircount: Count4,

    /// Maximum total reply size.
    pub maxcount: Count4,

    /// Bitmap of requested attributes for each entry.
    pub attr_request: Bitmap4,
}

impl ReadDir4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let cookie = r.read_u64()?;
        // TODO
        let cookieverf: [u8; VERIFIER_SIZE] = r
            .read_fixed_opaque(VERIFIER_SIZE)?
            .try_into()
            .map_err(|_| Nfsv4Error::InvalidData("expected 8 bytes".into()))?;
        let dircount = r.read_u32()?;
        let maxcount = r.read_u32()?;
        let attr_request = Bitmap4::decode(r)?;
        Ok(Self {
            cookie,
            cookieverf,
            dircount,
            maxcount,
            attr_request,
        })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.cookie)?;
        w.write_fixed_opaque(&self.cookieverf)?;
        w.write_u32(self.dircount)?;
        w.write_u32(self.maxcount)?;
        self.attr_request.encode(w)?;
        Ok(())
    }
}

/// RFC7531: entry4
///
/// Single directory entry returned by READDIR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry4 {
    /// Cookie used to continue reading after this entry.
    pub cookie: NfsCookie4,

    /// Entry name.
    pub name: Component4,

    /// Attributes associated with the entry.
    pub attrs: Fattr4,

    /// Next entry in the linked list.
    pub nextentry: Option<Box<Entry4>>,
}

impl Entry4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let cookie = r.read_u64()?;
        let name = r.read_string()?;
        let attrs = Fattr4::decode(r)?;
        let has_next = r.read_boolean()?;
        let nextentry = if has_next {
            Some(Box::new(Entry4::decode(r)?))
        } else {
            None
        };
        Ok(Self {
            cookie,
            name,
            attrs,
            nextentry,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.cookie)?;
        w.write_string(&self.name)?;
        self.attrs.encode(w)?;
        w.write_boolean(self.nextentry.is_some())?;
        if let Some(next) = &self.nextentry {
            next.encode(w)?;
        }
        Ok(())
    }
}

/// RFC7531: dirlist4
///
/// Directory listing returned by READDIR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirList4 {
    /// Head of the linked list of entries.
    pub entries: Option<Box<Entry4>>,

    /// Indicates whether the end of the directory was reached.
    pub eof: bool,
}

impl DirList4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let has_entries = r.read_boolean()?;
        let entries = if has_entries {
            Some(Box::new(Entry4::decode(r)?))
        } else {
            None
        };
        let eof = r.read_boolean()?;
        Ok(Self { entries, eof })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_boolean(self.entries.is_some())?;
        if let Some(entry) = &self.entries {
            entry.encode(w)?;
        }
        w.write_boolean(self.eof)?;
        Ok(())
    }
}

/// RFC7531: READDIR4resok
///
/// Successful result of the READDIR operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadDir4ResOk {
    /// Cookie verifier for subsequent READDIR calls.
    pub cookieverf: Verifier4,

    /// Returned directory entries.
    pub reply: DirList4,
}

impl ReadDir4ResOk {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let cookieverf: [u8; VERIFIER_SIZE] = r
            .read_fixed_opaque(VERIFIER_SIZE)?
            .try_into()
            .map_err(|_| Nfsv4Error::InvalidData("expected 8 bytes".into()))?;
        let reply = DirList4::decode(r)?;
        Ok(Self { cookieverf, reply })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_fixed_opaque(&self.cookieverf)?;
        self.reply.encode(w)?;
        Ok(())
    }
}

/// RFC7531: READDIR4res
///
/// Result of the READDIR operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadDir4Res {
    /// Operation succeeded.
    Ok(ReadDir4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

impl ReadDir4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let status = Stat4::try_from(r.read_i32()?)?;
        match status {
            Stat4::Ok => {
                let resok = ReadDir4ResOk::decode(r)?;
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

    fn sample_bitmap() -> Bitmap4 {
        let mut bm = Bitmap4::new();
        bm.insert(0);
        bm.insert(31);
        bm.insert(32);
        bm
    }

    fn sample_fattr4() -> Fattr4 {
        Fattr4 {
            attrmask: sample_bitmap(),
            attr_vals: vec![0x11, 0x22, 0x33, 0x44],
        }
    }

    fn sample_entry_chain() -> Entry4 {
        Entry4 {
            cookie: 1,
            name: "file1".to_string(),
            attrs: sample_fattr4(),
            nextentry: Some(Box::new(Entry4 {
                cookie: 2,
                name: "file2".to_string(),
                attrs: Fattr4 {
                    attrmask: Bitmap4::new(),
                    attr_vals: vec![0xaa, 0xbb],
                },
                nextentry: None,
            })),
        }
    }

    #[test]
    fn test_readdir4args_encode_decode() {
        let args = ReadDir4Args {
            cookie: 123,
            cookieverf: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
            dircount: 4096,
            maxcount: 8192,
            attr_request: sample_bitmap(),
        };

        let mut w = XdrWriter::new();
        args.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ReadDir4Args::decode(&mut r).unwrap();

        assert_eq!(args, decoded);
    }

    #[test]
    fn test_entry4_encode_decode_single() {
        let entry = Entry4 {
            cookie: 42,
            name: "single".to_string(),
            attrs: sample_fattr4(),
            nextentry: None,
        };

        let mut w = XdrWriter::new();
        entry.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Entry4::decode(&mut r).unwrap();

        assert_eq!(entry, decoded);
    }

    #[test]
    fn test_entry4_encode_decode_chain() {
        let entry = sample_entry_chain();

        let mut w = XdrWriter::new();
        entry.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = Entry4::decode(&mut r).unwrap();

        assert_eq!(entry, decoded);
    }

    #[test]
    fn test_dirlist4_encode_decode_empty() {
        let dirlist = DirList4 {
            entries: None,
            eof: true,
        };

        let mut w = XdrWriter::new();
        dirlist.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = DirList4::decode(&mut r).unwrap();

        assert_eq!(dirlist, decoded);
    }

    #[test]
    fn test_dirlist4_encode_decode_with_entries() {
        let dirlist = DirList4 {
            entries: Some(Box::new(sample_entry_chain())),
            eof: false,
        };

        let mut w = XdrWriter::new();
        dirlist.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = DirList4::decode(&mut r).unwrap();

        assert_eq!(dirlist, decoded);
    }

    #[test]
    fn test_readdir4resok_encode_decode() {
        let resok = ReadDir4ResOk {
            cookieverf: [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11],
            reply: DirList4 {
                entries: Some(Box::new(sample_entry_chain())),
                eof: false,
            },
        };

        let mut w = XdrWriter::new();
        resok.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ReadDir4ResOk::decode(&mut r).unwrap();

        assert_eq!(resok, decoded);
    }

    #[test]
    fn test_readdir4res_ok_encode_decode() {
        let res = ReadDir4Res::Ok(ReadDir4ResOk {
            cookieverf: [1, 2, 3, 4, 5, 6, 7, 8],
            reply: DirList4 {
                entries: Some(Box::new(sample_entry_chain())),
                eof: true,
            },
        });

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ReadDir4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }

    #[test]
    fn test_readdir4res_err_encode_decode() {
        let res = ReadDir4Res::Err(Stat4::Access);

        let mut w = XdrWriter::new();
        res.encode(&mut w).unwrap();

        let mut r = XdrReader::new(w.as_bytes());
        let decoded = ReadDir4Res::decode(&mut r).unwrap();

        assert_eq!(res, decoded);
    }
}

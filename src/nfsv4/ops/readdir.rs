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
        let attr_request = r.read_array(|r| r.read_u32())?;
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
        w.write_array(&self.attr_request, |w, v| w.write_u32(*v))?;
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

/// RFC7531: READDIR4res
///
/// Result of the READDIR operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadDir4Res {
    /// Operation succeeded.
    Ok(ReadDir4ResOk),

    /// Operation failed with an NFS error status.
    Err(NfsStat4),
}

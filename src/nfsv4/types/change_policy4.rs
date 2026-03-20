#![allow(dead_code)]

use super::*;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::nfsv4::types::Nfsv4Error;

/// RFC8881: change_policy4
///
/// Used to describe change attribute policy (major/minor versioning).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangePolicy4 {
    /// Major change policy value
    pub cp_major: u64,

    /// Minor change policy value
    pub cp_minor: u64,
}

impl ChangePolicy4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let cp_major = r.read_u64()?;
        let cp_minor = r.read_u64()?;
        Ok(Self { cp_major, cp_minor })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.cp_major)?;
        w.write_u64(self.cp_minor)?;
        Ok(())
    }
}

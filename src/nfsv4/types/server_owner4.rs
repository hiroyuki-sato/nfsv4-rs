#![allow(dead_code)]

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::nfsv4::types::Nfsv4Error;

/// RFC8881 Section 2.5: server_owner4
///
/// Identifies the server instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerOwner4 {
    /// Minor identifier (usually server incarnation)
    pub so_minor_id: u64,

    /// Major identifier (opaque server identity)
    pub so_major_id: Vec<u8>,
}

impl ServerOwner4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let so_minor_id = r.read_u64()?;
        let so_major_id = r.read_opaque()?; // <NFS4_OPAQUE_LIMIT>
        Ok(Self {
            so_minor_id,
            so_major_id,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_u64(self.so_minor_id)?;
        w.write_opaque(&self.so_major_id)?;
        Ok(())
    }
}

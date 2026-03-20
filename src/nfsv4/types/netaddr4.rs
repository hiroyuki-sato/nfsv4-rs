#![allow(dead_code)]

use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC7530: netaddr4
///
/// Network address structure used in callbacks and referrals.
/// See also rpcb in RFC 1833.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetAddr4 {
    /// Network identifier (e.g., "tcp", "udp")
    pub na_r_netid: String,

    /// Universal address (transport-specific format)
    pub na_r_addr: String,
}

impl NetAddr4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let na_r_netid = r.read_string()?;
        let na_r_addr = r.read_string()?;
        Ok(Self {
            na_r_netid,
            na_r_addr,
        })
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_string(&self.na_r_netid)?;
        w.write_string(&self.na_r_addr)?;
        Ok(())
    }
}

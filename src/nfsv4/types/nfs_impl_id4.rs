#![allow(dead_code)]

use crate::nfsv4::types::NfsTime4;
use crate::nfsv4::types::Nfsv4Error;

use xdr_rs::XdrError;
use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

/// RFC8881 Section 3.3.21: nfs_impl_id4
///
/// Identifies the implementation (vendor/domain/name/date).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NfsImplId4 {
    /// Implementation domain (case-insensitive)
    pub nii_domain: String,

    /// Implementation name (case-sensitive)
    pub nii_name: String,

    /// Implementation date
    pub nii_date: NfsTime4,
}

impl NfsImplId4 {
    pub fn decode_xdr(r: &mut XdrReader) -> Result<Self, XdrError> {
        let nii_domain = r.read_string()?;
        let nii_name = r.read_string()?;
        let nii_date = NfsTime4::decode_xdr(r)?;
        Ok(Self {
            nii_domain,
            nii_name,
            nii_date,
        })
    }

    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self::decode_xdr(r)?)
    }

    pub fn encode_xdr(&self, w: &mut XdrWriter) -> Result<(), XdrError> {
        w.write_string(&self.nii_domain)?;
        w.write_string(&self.nii_name)?;
        self.nii_date.encode_xdr(w)?;
        Ok(())
    }

    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        self.encode_xdr(w)?;
        Ok(())
    }
}

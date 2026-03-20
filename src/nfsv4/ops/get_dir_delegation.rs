use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::Bitmap4;
use crate::nfsv4::types::NfsTime4;
use crate::nfsv4::types::Stat4;
use crate::nfsv4::types::StateId4;
use crate::nfsv4::types::Verifier4;

/// RFC8881 Section 18.39.1: attr_notice4
pub type AttrNotice4 = NfsTime4;

/// RFC8881 Section 18.39.1: GET_DIR_DELEGATION4args
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDirDelegation4Args {
    /// CURRENT_FH: delegated directory
    pub gdda_signal_deleg_avail: bool,
    pub gdda_notification_types: Bitmap4,
    pub gdda_child_attr_delay: AttrNotice4,
    pub gdda_dir_attr_delay: AttrNotice4,
    pub gdda_child_attributes: Bitmap4,
    pub gdda_dir_attributes: Bitmap4,
}

/// RFC8881 Section 18.39.2: GET_DIR_DELEGATION4resok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDirDelegation4ResOk {
    pub gddr_cookieverf: Verifier4,
    /// Stateid for get_dir_delegation
    pub gddr_stateid: StateId4,
    /// Which notifications can the server support
    pub gddr_notification: Bitmap4,
    pub gddr_child_attributes: Bitmap4,
    pub gddr_dir_attributes: Bitmap4,
}

/// RFC8881 Section 18.39.2: gddrnf4_status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Gddrnf4Status {
    Gdd4Ok = 0,
    Gdd4Unavail = 1,
}

/// RFC8881 Section 18.39.2: GET_DIR_DELEGATION4res_non_fatal
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetDirDelegation4ResNonFatal {
    Ok(GetDirDelegation4ResOk),
    Unavail(bool),
}

/// RFC8881 Section 18.39.2: GET_DIR_DELEGATION4res
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetDirDelegation4Res {
    Ok(GetDirDelegation4ResNonFatal),
    Err(Stat4),
}

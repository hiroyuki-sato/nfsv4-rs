#![allow(dead_code)]

use crate::nfsv4::types::*;

/// RFC7531: CB_GETATTR4args
///
/// Arguments for the CB_GETATTR callback operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CbGetAttr4Args {
    /// Filehandle of the object.
    pub fh: NfsFh4,

    /// Bitmap of requested attributes.
    pub attr_request: Bitmap4,
}

/// RFC7531: CB_GETATTR4resok
///
/// Successful result of the CB_GETATTR callback operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CbGetAttr4ResOk {
    /// Returned object attributes.
    pub obj_attributes: Fattr4,
}

/// RFC7531: CB_GETATTR4res
///
/// Result of the CB_GETATTR callback operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CbGetAttr4Res {
    /// Operation succeeded.
    Ok(CbGetAttr4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

/// RFC7531: CB_RECALL4args
///
/// Arguments for the CB_RECALL callback operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CbRecall4Args {
    /// Delegation state ID to be recalled.
    pub stateid: StateId4,

    /// Indicates whether the file should be truncated.
    pub truncate: bool,

    /// Filehandle of the recalled file.
    pub fh: NfsFh4,
}

/// RFC7531: CB_RECALL4res
///
/// Result of the CB_RECALL callback operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CbRecall4Res {
    /// NFS operation status.
    pub status: Stat4,
}

/// RFC7531: CB_ILLEGAL4res
///
/// Result returned for an illegal callback operation number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CbIllegal4Res {
    /// NFS operation status.
    pub status: Stat4,
}

/// RFC7531: nfs_cb_opnum4
///
/// Callback operation numbers used in CB_COMPOUND requests and responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NfsCbOpnum4 {
    CbGetAttr = 3,
    CbRecall = 4,
    CbIllegal = 10044,
}

/// RFC7531: nfs_cb_argop4
///
/// Callback operation arguments used in a CB_COMPOUND request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NfsCbArgOp4 {
    CbGetAttr(CbGetAttr4Args),
    CbRecall(CbRecall4Args),

    /// Illegal callback operation.
    CbIllegal,
}

/// RFC7531: nfs_cb_resop4
///
/// Callback operation results used in a CB_COMPOUND response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NfsCbResOp4 {
    CbGetAttr(CbGetAttr4Res),
    CbRecall(CbRecall4Res),
    CbIllegal(CbIllegal4Res),
}

/// RFC7531: CB_COMPOUND4args
///
/// Top-level arguments for an NFSv4 callback COMPOUND request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CbCompound4Args {
    /// Opaque tag string supplied by the server.
    pub tag: String,

    /// Minor version of the NFSv4 protocol.
    pub minorversion: u32,

    /// Callback identifier established by SETCLIENTID.
    pub callback_ident: u32,

    /// Sequence of callback operations in the request.
    pub argarray: Vec<NfsCbArgOp4>,
}

/// RFC7531: CB_COMPOUND4res
///
/// Top-level result for an NFSv4 callback COMPOUND response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CbCompound4Res {
    /// Final status of the callback COMPOUND request.
    pub status: Stat4,

    /// Tag echoed back in the response.
    pub tag: String,

    /// Sequence of callback operation results.
    pub resarray: Vec<NfsCbResOp4>,
}

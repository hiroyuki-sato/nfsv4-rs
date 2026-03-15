#![allow(dead_code)]

use crate::nfsv4::ops::*;
use crate::nfsv4::types::*;

/// RFC7531: nfs_opnum4
///
/// Operation numbers used in COMPOUND requests and responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NfsOpnum4 {
    Access = 3,
    Close = 4,
    Commit = 5,
    Create = 6,
    DelegPurge = 7,
    DelegReturn = 8,
    GetAttr = 9,
    GetFh = 10,
    Link = 11,
    Lock = 12,
    LockT = 13,
    LockU = 14,
    Lookup = 15,
    LookupP = 16,
    NVerify = 17,
    Open = 18,
    OpenAttr = 19,
    OpenConfirm = 20,
    OpenDowngrade = 21,
    PutFh = 22,
    PutPubFh = 23,
    PutRootFh = 24,
    Read = 25,
    ReadDir = 26,
    ReadLink = 27,
    Remove = 28,
    Rename = 29,
    Renew = 30,
    RestoreFh = 31,
    SaveFh = 32,
    SecInfo = 33,
    SetAttr = 34,
    SetClientId = 35,
    SetClientIdConfirm = 36,
    Verify = 37,
    Write = 38,
    ReleaseLockOwner = 39,

    /// Returned when an undefined operation is encountered.
    Illegal = 10044,
}

/// RFC7531: nfs_argop4
///
/// Operation arguments used in a COMPOUND request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NfsArgOp4 {
    Access(Access4Args),
    Close(Close4Args),
    Commit(Commit4Args),
    Create(Create4Args),
    DelegPurge(DelegPurge4Args),
    DelegReturn(DelegReturn4Args),
    GetAttr(GetAttr4Args),

    /// GETFH has no arguments.
    GetFh,

    Link(Link4Args),
    Lock(Lock4Args),
    LockT(LockT4Args),
    LockU(LockU4Args),
    Lookup(Lookup4Args),

    /// LOOKUPP has no arguments.
    LookupP,

    NVerify(NVerify4Args),
    Open(Open4Args),
    OpenAttr(OpenAttr4Args),
    OpenConfirm(OpenConfirm4Args),
    OpenDowngrade(OpenDowngrade4Args),
    PutFh(PutFh4Args),

    /// PUTPUBFH has no arguments.
    PutPubFh,

    /// PUTROOTFH has no arguments.
    PutRootFh,

    Read(Read4Args),
    ReadDir(ReadDir4Args),

    /// READLINK has no arguments.
    ReadLink,

    Remove(Remove4Args),
    Rename(Rename4Args),
    Renew(Renew4Args),

    /// RESTOREFH has no arguments.
    RestoreFh,

    /// SAVEFH has no arguments.
    SaveFh,

    SecInfo(SecInfo4Args),
    SetAttr(SetAttr4Args),
    SetClientId(SetClientId4Args),
    SetClientIdConfirm(SetClientIdConfirm4Args),
    Verify(Verify4Args),
    Write(Write4Args),
    ReleaseLockOwner(ReleaseLockOwner4Args),

    /// Illegal operation.
    Illegal,
}

/// RFC7531: nfs_resop4
///
/// Operation results used in a COMPOUND response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NfsResOp4 {
    Access(Access4Res),
    Close(Close4Res),
    Commit(Commit4Res),
    Create(Create4Res),
    DelegPurge(DelegPurge4Res),
    DelegReturn(DelegReturn4Res),
    GetAttr(GetAttr4Res),
    GetFh(GetFh4Res),
    Link(Link4Res),
    Lock(Lock4Res),
    LockT(LockT4Res),
    LockU(LockU4Res),
    Lookup(Lookup4Res),
    LookupP(LookupP4Res),
    NVerify(NVerify4Res),
    Open(Open4Res),
    OpenAttr(OpenAttr4Res),
    OpenConfirm(OpenConfirm4Res),
    OpenDowngrade(OpenDowngrade4Res),
    PutFh(PutFh4Res),
    PutPubFh(PutPubFh4Res),
    PutRootFh(PutRootFh4Res),
    Read(Read4Res),
    ReadDir(ReadDir4Res),
    ReadLink(ReadLink4Res),
    Remove(Remove4Res),
    Rename(Rename4Res),
    Renew(Renew4Res),
    RestoreFh(RestoreFh4Res),
    SaveFh(SaveFh4Res),
    SecInfo(SecInfo4Res),
    SetAttr(SetAttr4Res),
    SetClientId(SetClientId4Res),
    SetClientIdConfirm(SetClientIdConfirm4Res),
    Verify(Verify4Res),
    Write(Write4Res),
    ReleaseLockOwner(ReleaseLockOwner4Res),
    Illegal(Illegal4Res),
}

/// RFC7531: COMPOUND4args
///
/// Top-level arguments for an NFSv4 COMPOUND request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compound4Args {
    /// Opaque tag string supplied by the client.
    pub tag: String,

    /// Minor version of the NFSv4 protocol.
    pub minorversion: u32,

    /// Sequence of operations in the COMPOUND request.
    pub argarray: Vec<NfsArgOp4>,
}

/// RFC7531: COMPOUND4res
///
/// Top-level result for an NFSv4 COMPOUND response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compound4Res {
    /// Final status of the COMPOUND request.
    pub status: NfsStat4,

    /// Tag echoed back by the server.
    pub tag: String,

    /// Sequence of operation results in the COMPOUND response.
    pub resarray: Vec<NfsResOp4>,
}

//
// Remote file service routines
//
// program NFS4_PROGRAM {
//         version NFS_V4 {
//                 void
//                         NFSPROC4_NULL(void) = 0;
//
//                 COMPOUND4res
//                         NFSPROC4_COMPOUND(COMPOUND4args) = 1;
//
//         } = 4;
// } = 100003;

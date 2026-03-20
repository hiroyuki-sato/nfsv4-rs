#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
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

    /// NFSv4.1
    BackchannelCtl = 40,
    BindConnToSession = 41,
    ExchangeId = 42,
    CreateSession = 43,
    DestroySession = 44,
    FreeStateId = 45,
    GetDirDelegation = 46,
    GetDeviceInfo = 47,
    GetDeviceList = 48,
    LayoutCommit = 49,
    LayoutGet = 50,
    LayoutReturn = 51,
    SecInfoNoName = 52,
    Sequence = 53,
    SetSsv = 54,
    TestStateId = 55,
    WantDelegation = 56,
    DestroyClientId = 57,
    ReclaimComplete = 58,

    /// Returned when an undefined operation is encountered.
    Illegal = 10044,
}

impl TryFrom<i32> for NfsOpnum4 {
    type Error = Nfsv4Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            3 => Ok(NfsOpnum4::Access),
            4 => Ok(NfsOpnum4::Close),
            5 => Ok(NfsOpnum4::Commit),
            6 => Ok(NfsOpnum4::Create),
            7 => Ok(NfsOpnum4::DelegPurge),
            8 => Ok(NfsOpnum4::DelegReturn),
            9 => Ok(NfsOpnum4::GetAttr),
            10 => Ok(NfsOpnum4::GetFh),
            11 => Ok(NfsOpnum4::Link),
            12 => Ok(NfsOpnum4::Lock),
            13 => Ok(NfsOpnum4::LockT),
            14 => Ok(NfsOpnum4::LockU),
            15 => Ok(NfsOpnum4::Lookup),
            16 => Ok(NfsOpnum4::LookupP),
            17 => Ok(NfsOpnum4::NVerify),
            18 => Ok(NfsOpnum4::Open),
            19 => Ok(NfsOpnum4::OpenAttr),
            20 => Ok(NfsOpnum4::OpenConfirm),
            21 => Ok(NfsOpnum4::OpenDowngrade),
            22 => Ok(NfsOpnum4::PutFh),
            23 => Ok(NfsOpnum4::PutPubFh),
            24 => Ok(NfsOpnum4::PutRootFh),
            25 => Ok(NfsOpnum4::Read),
            26 => Ok(NfsOpnum4::ReadDir),
            27 => Ok(NfsOpnum4::ReadLink),
            28 => Ok(NfsOpnum4::Remove),
            29 => Ok(NfsOpnum4::Rename),
            30 => Ok(NfsOpnum4::Renew),
            31 => Ok(NfsOpnum4::RestoreFh),
            32 => Ok(NfsOpnum4::SaveFh),
            33 => Ok(NfsOpnum4::SecInfo),
            34 => Ok(NfsOpnum4::SetAttr),
            35 => Ok(NfsOpnum4::SetClientId),
            36 => Ok(NfsOpnum4::SetClientIdConfirm),
            37 => Ok(NfsOpnum4::Verify),
            38 => Ok(NfsOpnum4::Write),
            39 => Ok(NfsOpnum4::ReleaseLockOwner),
            10044 => Ok(NfsOpnum4::Illegal),
            // NFSv4.1
            40 => Ok(NfsOpnum4::BackchannelCtl),
            41 => Ok(NfsOpnum4::BindConnToSession),
            42 => Ok(NfsOpnum4::ExchangeId),
            43 => Ok(NfsOpnum4::CreateSession),
            44 => Ok(NfsOpnum4::DestroySession),
            45 => Ok(NfsOpnum4::FreeStateId),
            46 => Ok(NfsOpnum4::GetDirDelegation),
            47 => Ok(NfsOpnum4::GetDeviceInfo),
            48 => Ok(NfsOpnum4::GetDeviceList),
            49 => Ok(NfsOpnum4::LayoutCommit),
            50 => Ok(NfsOpnum4::LayoutGet),
            51 => Ok(NfsOpnum4::LayoutReturn),
            52 => Ok(NfsOpnum4::SecInfoNoName),
            53 => Ok(NfsOpnum4::Sequence),
            54 => Ok(NfsOpnum4::SetSsv),
            55 => Ok(NfsOpnum4::TestStateId),
            56 => Ok(NfsOpnum4::WantDelegation),
            57 => Ok(NfsOpnum4::DestroyClientId),
            58 => Ok(NfsOpnum4::ReclaimComplete),
            _ => Err(Nfsv4Error::InvalidNfsOpnum4(v)),
        }
    }
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

    // NFSv4.1
    // BackchannelCtl(BackchannelCtl4Args), // TODO
    // BindConnToSession(BindConnToSession4Args),
    // ExchangeId(ExchangeId4Args), // TODO
    CreateSession(CreateSession4Args),
    DestroySession(DestroySession4Args),
    FreeStateId(FreeStateId4Args),
    GetDirDelegation(GetDirDelegation4Args),
    GetDeviceInfo(GetDeviceInfo4Args),
    GetDeviceList(GetDeviceList4Args),
    LayoutCommit(LayoutCommit4Args),
    LayoutGet(LayoutGet4Args),
    LayoutReturn(LayoutReturn4Args),
    SecInfoNoName(SecInfoNoName4Args),
    Sequence(Sequence4Args),
    SetSsv(SetSsv4Args),
    TestStateId(TestStateId4Args),
    WantDelegation(WantDelegation4Args),
    DestroyClientId(DestroyClientId4Args),
    ReclaimComplete(ReclaimComplete4Args),

    /// Illegal operation.
    Illegal,
}

impl NfsArgOp4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let op = NfsOpnum4::try_from(r.read_i32()?)?;
        match op {
            NfsOpnum4::PutPubFh => Ok(NfsArgOp4::PutPubFh),
            _ => {
                unimplemented!();
            }
        }
    }
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
    pub status: Stat4,

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

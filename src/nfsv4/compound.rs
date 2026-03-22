#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::ops::*;
use crate::nfsv4::types::*;
use crate::xdr_ext::{read_array_nfs, write_array_nfs};

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

            10044 => Ok(NfsOpnum4::Illegal),
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
    BackchannelCtl(BackchannelCtl4Args),
    BindConnToSession(BindConnToSession4Args),
    ExchangeId(ExchangeId4Args),
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
            // NfsOpnum4::Access => Ok(NfsArgOp4::Access(Access4Args::decode(r)?)),
            // NfsOpnum4::Close => Ok(NfsArgOp4::Close(Close4Args::decode(r)?)),
            // NfsOpnum4::Commit => Ok(NfsArgOp4::Commit(Commit4Args::decode(r)?)),
            // NfsOpnum4::Create => Ok(NfsArgOp4::Create(Create4Args::decode(r)?)),
            // NfsOpnum4::DelegPurge => Ok(NfsArgOp4::DelegPurge(DelegPurge4Args::decode(r)?)),
            // NfsOpnum4::DelegReturn => Ok(NfsArgOp4::DelegReturn(DelegReturn4Args::decode(r)?)),
            // NfsOpnum4::GetAttr => Ok(NfsArgOp4::GetAttr(GetAttr4Args::decode(r)?)),
            // NfsOpnum4::GetFh => Ok(NfsArgOp4::GetFh),
            // NfsOpnum4::Link => Ok(NfsArgOp4::Link(Link4Args::decode(r)?)),
            // NfsOpnum4::Lock => Ok(NfsArgOp4::Lock(Lock4Args::decode(r)?)),
            // NfsOpnum4::LockT => Ok(NfsArgOp4::LockT(LockT4Args::decode(r)?)),
            // NfsOpnum4::LockU => Ok(NfsArgOp4::LockU(LockU4Args::decode(r)?)),
            // NfsOpnum4::Lookup => Ok(NfsArgOp4::Lookup(Lookup4Args::decode(r)?)),
            // NfsOpnum4::LookupP => Ok(NfsArgOp4::LookupP),
            // NfsOpnum4::NVerify => Ok(NfsArgOp4::NVerify(NVerify4Args::decode(r)?)),
            // NfsOpnum4::Open => Ok(NfsArgOp4::Open(Open4Args::decode(r)?)),
            // NfsOpnum4::OpenAttr => Ok(NfsArgOp4::OpenAttr(OpenAttr4Args::decode(r)?)),
            // NfsOpnum4::OpenConfirm => Ok(NfsArgOp4::OpenConfirm(OpenConfirm4Args::decode(r)?)),
            // NfsOpnum4::OpenDowngrade => {
            //     Ok(NfsArgOp4::OpenDowngrade(OpenDowngrade4Args::decode(r)?))
            // }
            // NfsOpnum4::PutFh => Ok(NfsArgOp4::PutFh(PutFh4Args::decode(r)?)),
            // NfsOpnum4::PutPubFh => Ok(NfsArgOp4::PutPubFh),
            NfsOpnum4::PutRootFh => Ok(NfsArgOp4::PutRootFh),
            // NfsOpnum4::Read => Ok(NfsArgOp4::Read(Read4Args::decode(r)?)),
            NfsOpnum4::ReadDir => Ok(NfsArgOp4::ReadDir(ReadDir4Args::decode(r)?)),
            // NfsOpnum4::ReadLink => Ok(NfsArgOp4::ReadLink),
            // NfsOpnum4::Remove => Ok(NfsArgOp4::Remove(Remove4Args::decode(r)?)),
            // NfsOpnum4::Rename => Ok(NfsArgOp4::Rename(Rename4Args::decode(r)?)),
            // NfsOpnum4::Renew => Ok(NfsArgOp4::Renew(Renew4Args::decode(r)?)),
            // NfsOpnum4::RestoreFh => Ok(NfsArgOp4::RestoreFh),
            // NfsOpnum4::SaveFh => Ok(NfsArgOp4::SaveFh),
            NfsOpnum4::SecInfo => Ok(NfsArgOp4::SecInfo(SecInfo4Args::decode(r)?)),
            // NfsOpnum4::SetAttr => Ok(NfsArgOp4::SetAttr(SetAttr4Args::decode(r)?)),
            // NfsOpnum4::SetClientId => Ok(NfsArgOp4::SetClientId(SetClientId4Args::decode(r)?)),
            // NfsOpnum4::SetClientIdConfirm => Ok(NfsArgOp4::SetClientIdConfirm(
            //     SetClientIdConfirm4Args::decode(r)?,
            // )),
            // NfsOpnum4::Verify => Ok(NfsArgOp4::Verify(Verify4Args::decode(r)?)),
            // NfsOpnum4::Write => Ok(NfsArgOp4::Write(Write4Args::decode(r)?)),
            // NfsOpnum4::ReleaseLockOwner => Ok(NfsArgOp4::ReleaseLockOwner(
            //     ReleaseLockOwner4Args::decode(r)?,
            // )),

            // NFSv4.1
            NfsOpnum4::BackchannelCtl => {
                Ok(NfsArgOp4::BackchannelCtl(BackchannelCtl4Args::decode(r)?))
            }
            // NfsOpnum4::BindConnToSession => Ok(NfsArgOp4::BindConnToSession(
            //     BindConnToSession4Args::decode(r)?,
            // )),
            NfsOpnum4::ExchangeId => Ok(NfsArgOp4::ExchangeId(ExchangeId4Args::decode(r)?)),
            NfsOpnum4::CreateSession => {
                Ok(NfsArgOp4::CreateSession(CreateSession4Args::decode(r)?))
            }
            // NfsOpnum4::DestroySession => {
            //     Ok(NfsArgOp4::DestroySession(DestroySession4Args::decode(r)?))
            // }
            // NfsOpnum4::FreeStateId => Ok(NfsArgOp4::FreeStateId(FreeStateId4Args::decode(r)?)),
            // NfsOpnum4::GetDirDelegation => Ok(NfsArgOp4::GetDirDelegation(
            //     GetDirDelegation4Args::decode(r)?,
            // )),
            // NfsOpnum4::GetDeviceInfo => {
            //     Ok(NfsArgOp4::GetDeviceInfo(GetDeviceInfo4Args::decode(r)?))
            // }
            // NfsOpnum4::GetDeviceList => {
            //     Ok(NfsArgOp4::GetDeviceList(GetDeviceList4Args::decode(r)?))
            // }
            // NfsOpnum4::LayoutCommit => Ok(NfsArgOp4::LayoutCommit(LayoutCommit4Args::decode(r)?)),
            // NfsOpnum4::LayoutGet => Ok(NfsArgOp4::LayoutGet(LayoutGet4Args::decode(r)?)),
            // NfsOpnum4::LayoutReturn => Ok(NfsArgOp4::LayoutReturn(LayoutReturn4Args::decode(r)?)),
            // NfsOpnum4::SecInfoNoName => {
            //     Ok(NfsArgOp4::SecInfoNoName(SecInfoNoName4Args::decode(r)?))
            // }
            NfsOpnum4::Sequence => Ok(NfsArgOp4::Sequence(Sequence4Args::decode(r)?)),
            // NfsOpnum4::SetSsv => Ok(NfsArgOp4::SetSsv(SetSsv4Args::decode(r)?)),
            // NfsOpnum4::TestStateId => Ok(NfsArgOp4::TestStateId(TestStateId4Args::decode(r)?)),
            // NfsOpnum4::WantDelegation => {
            //     Ok(NfsArgOp4::WantDelegation(WantDelegation4Args::decode(r)?))
            // }
            // NfsOpnum4::DestroyClientId => {
            //     Ok(NfsArgOp4::DestroyClientId(DestroyClientId4Args::decode(r)?))
            // }
            // NfsOpnum4::ReclaimComplete => {
            //     Ok(NfsArgOp4::ReclaimComplete(ReclaimComplete4Args::decode(r)?))
            // }
            NfsOpnum4::Illegal => Ok(NfsArgOp4::Illegal),
            _ => Err(Nfsv4Error::NotImplementedOp(op)),
        }
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            // NfsArgOp4::Access(arg) => {
            //     w.write_i32(NfsOpnum4::Access as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Close(arg) => {
            //     w.write_i32(NfsOpnum4::Close as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Commit(arg) => {
            //     w.write_i32(NfsOpnum4::Commit as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Create(arg) => {
            //     w.write_i32(NfsOpnum4::Create as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::DelegPurge(arg) => {
            //     w.write_i32(NfsOpnum4::DelegPurge as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::DelegReturn(arg) => {
            //     w.write_i32(NfsOpnum4::DelegReturn as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::GetAttr(arg) => {
            //     w.write_i32(NfsOpnum4::GetAttr as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::GetFh => {
            //     w.write_i32(NfsOpnum4::GetFh as i32)?;
            // }
            // NfsArgOp4::Link(arg) => {
            //     w.write_i32(NfsOpnum4::Link as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Lock(arg) => {
            //     w.write_i32(NfsOpnum4::Lock as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::LockT(arg) => {
            //     w.write_i32(NfsOpnum4::LockT as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::LockU(arg) => {
            //     w.write_i32(NfsOpnum4::LockU as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Lookup(arg) => {
            //     w.write_i32(NfsOpnum4::Lookup as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::LookupP => {
            //     w.write_i32(NfsOpnum4::LookupP as i32)?;
            // }
            // NfsArgOp4::NVerify(arg) => {
            //     w.write_i32(NfsOpnum4::NVerify as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Open(arg) => {
            //     w.write_i32(NfsOpnum4::Open as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::OpenAttr(arg) => {
            //     w.write_i32(NfsOpnum4::OpenAttr as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::OpenConfirm(arg) => {
            //     w.write_i32(NfsOpnum4::OpenConfirm as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::OpenDowngrade(arg) => {
            //     w.write_i32(NfsOpnum4::OpenDowngrade as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::PutFh(arg) => {
            //     w.write_i32(NfsOpnum4::PutFh as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::PutPubFh => {
            //     w.write_i32(NfsOpnum4::PutPubFh as i32)?;
            // }
            NfsArgOp4::PutRootFh => {
                w.write_i32(NfsOpnum4::PutRootFh as i32)?;
            }
            // NfsArgOp4::Read(arg) => {
            //     w.write_i32(NfsOpnum4::Read as i32)?;
            //     arg.encode(w)?;
            // }
            NfsArgOp4::ReadDir(arg) => {
                w.write_i32(NfsOpnum4::ReadDir as i32)?;
                arg.encode(w)?;
            }
            // NfsArgOp4::ReadLink => {
            //     w.write_i32(NfsOpnum4::ReadLink as i32)?;
            // }
            // NfsArgOp4::Remove(arg) => {
            //     w.write_i32(NfsOpnum4::Remove as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Rename(arg) => {
            //     w.write_i32(NfsOpnum4::Rename as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Renew(arg) => {
            //     w.write_i32(NfsOpnum4::Renew as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::RestoreFh => {
            //     w.write_i32(NfsOpnum4::RestoreFh as i32)?;
            // }
            // NfsArgOp4::SaveFh => {
            //     w.write_i32(NfsOpnum4::SaveFh as i32)?;
            // }
            NfsArgOp4::SecInfo(arg) => {
                w.write_i32(NfsOpnum4::SecInfo as i32)?;
                arg.encode(w)?;
            }
            // NfsArgOp4::SetAttr(arg) => {
            //     w.write_i32(NfsOpnum4::SetAttr as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::SetClientId(arg) => {
            //     w.write_i32(NfsOpnum4::SetClientId as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::SetClientIdConfirm(arg) => {
            //     w.write_i32(NfsOpnum4::SetClientIdConfirm as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Verify(arg) => {
            //     w.write_i32(NfsOpnum4::Verify as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::Write(arg) => {
            //     w.write_i32(NfsOpnum4::Write as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::ReleaseLockOwner(arg) => {
            //     w.write_i32(NfsOpnum4::ReleaseLockOwner as i32)?;
            //     arg.encode(w)?;
            // }

            // NFSv4.1
            NfsArgOp4::BackchannelCtl(arg) => {
                w.write_i32(NfsOpnum4::BackchannelCtl as i32)?;
                arg.encode(w)?;
            }
            // NfsArgOp4::BindConnToSession(arg) => {
            //     w.write_i32(NfsOpnum4::BindConnToSession as i32)?;
            //     arg.encode(w)?;
            // }
            NfsArgOp4::ExchangeId(arg) => {
                w.write_i32(NfsOpnum4::ExchangeId as i32)?;
                arg.encode(w)?;
            }
            NfsArgOp4::CreateSession(arg) => {
                w.write_i32(NfsOpnum4::CreateSession as i32)?;
                arg.encode(w)?;
            }
            // NfsArgOp4::DestroySession(arg) => {
            //     w.write_i32(NfsOpnum4::DestroySession as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::FreeStateId(arg) => {
            //     w.write_i32(NfsOpnum4::FreeStateId as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::GetDirDelegation(arg) => {
            //     w.write_i32(NfsOpnum4::GetDirDelegation as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::GetDeviceInfo(arg) => {
            //     w.write_i32(NfsOpnum4::GetDeviceInfo as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::GetDeviceList(arg) => {
            //     w.write_i32(NfsOpnum4::GetDeviceList as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::LayoutCommit(arg) => {
            //     w.write_i32(NfsOpnum4::LayoutCommit as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::LayoutGet(arg) => {
            //     w.write_i32(NfsOpnum4::LayoutGet as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::LayoutReturn(arg) => {
            //     w.write_i32(NfsOpnum4::LayoutReturn as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::SecInfoNoName(arg) => {
            //     w.write_i32(NfsOpnum4::SecInfoNoName as i32)?;
            //     arg.encode(w)?;
            // }
            NfsArgOp4::Sequence(arg) => {
                w.write_i32(NfsOpnum4::Sequence as i32)?;
                arg.encode(w)?;
            }
            // NfsArgOp4::SetSsv(arg) => {
            //     w.write_i32(NfsOpnum4::SetSsv as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::TestStateId(arg) => {
            //     w.write_i32(NfsOpnum4::TestStateId as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::WantDelegation(arg) => {
            //     w.write_i32(NfsOpnum4::WantDelegation as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::DestroyClientId(arg) => {
            //     w.write_i32(NfsOpnum4::DestroyClientId as i32)?;
            //     arg.encode(w)?;
            // }
            // NfsArgOp4::ReclaimComplete(arg) => {
            //     w.write_i32(NfsOpnum4::ReclaimComplete as i32)?;
            //     arg.encode(w)?;
            // }
            NfsArgOp4::Illegal => {
                w.write_i32(NfsOpnum4::Illegal as i32)?;
            }
            _ => return Err(Nfsv4Error::InvalidEnumValue(NfsOpnum4::Illegal as i32)),
        }
        Ok(())
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

    // NFSv4.1
    BackchannelCtl(BackchannelCtl4Res),
    BindConnToSession(BindConnToSession4Res),
    ExchangeId(ExchangeId4Res),
    CreateSession(CreateSession4Res),
    DestroySession(DestroySession4Res),
    FreeStateId(FreeStateId4Res),
    GetDirDelegation(GetDirDelegation4Res),
    GetDeviceInfo(GetDeviceInfo4Res),
    GetDeviceList(GetDeviceList4Res),
    LayoutCommit(LayoutCommit4Res),
    LayoutGet(LayoutGet4Res),
    LayoutReturn(LayoutReturn4Res),
    SecInfoNoName(SecInfoNoName4Res),
    Sequence(Sequence4Res),
    SetSsv(SetSsv4Res),
    TestStateId(TestStateId4Res),
    WantDelegation(WantDelegation4Res),
    DestroyClientId(DestroyClientId4Res),
    ReclaimComplete(ReclaimComplete4Res),

    Illegal(Illegal4Res),
}

impl NfsResOp4 {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        let op = NfsOpnum4::try_from(r.read_i32()?)?;
        match op {
            // NfsOpnum4::Access => Ok(NfsResOp4::Access(Access4Res::decode(r)?)),
            // NfsOpnum4::Close => Ok(NfsResOp4::Close(Close4Res::decode(r)?)),
            // NfsOpnum4::Commit => Ok(NfsResOp4::Commit(Commit4Res::decode(r)?)),
            // NfsOpnum4::Create => Ok(NfsResOp4::Create(Create4Res::decode(r)?)),
            // NfsOpnum4::DelegPurge => Ok(NfsResOp4::DelegPurge(DelegPurge4Res::decode(r)?)),
            // NfsOpnum4::DelegReturn => Ok(NfsResOp4::DelegReturn(DelegReturn4Res::decode(r)?)),
            NfsOpnum4::GetAttr => Ok(NfsResOp4::GetAttr(GetAttr4Res::decode(r)?)),
            // NfsOpnum4::GetFh => Ok(NfsResOp4::GetFh(GetFh4Res::decode(r)?)),
            // NfsOpnum4::Link => Ok(NfsResOp4::Link(Link4Res::decode(r)?)),
            // NfsOpnum4::Lock => Ok(NfsResOp4::Lock(Lock4Res::decode(r)?)),
            // NfsOpnum4::LockT => Ok(NfsResOp4::LockT(LockT4Res::decode(r)?)),
            // NfsOpnum4::LockU => Ok(NfsResOp4::LockU(LockU4Res::decode(r)?)),
            NfsOpnum4::Lookup => Ok(NfsResOp4::Lookup(Lookup4Res::decode(r)?)),
            // NfsOpnum4::LookupP => Ok(NfsResOp4::LookupP(LookupP4Res::decode(r)?)),
            // NfsOpnum4::NVerify => Ok(NfsResOp4::NVerify(NVerify4Res::decode(r)?)),
            // NfsOpnum4::Open => Ok(NfsResOp4::Open(Open4Res::decode(r)?)),
            // NfsOpnum4::OpenAttr => Ok(NfsResOp4::OpenAttr(OpenAttr4Res::decode(r)?)),
            // NfsOpnum4::OpenConfirm => Ok(NfsResOp4::OpenConfirm(OpenConfirm4Res::decode(r)?)),
            // NfsOpnum4::OpenDowngrade => Ok(NfsResOp4::OpenDowngrade(OpenDowngrade4Res::decode(r)?)),
            // NfsOpnum4::PutFh => Ok(NfsResOp4::PutFh(PutFh4Res::decode(r)?)),
            // NfsOpnum4::PutPubFh => Ok(NfsResOp4::PutPubFh(PutPubFh4Res::decode(r)?)),
            NfsOpnum4::PutRootFh => Ok(NfsResOp4::PutRootFh(PutRootFh4Res::decode(r)?)),
            // NfsOpnum4::Read => Ok(NfsResOp4::Read(Read4Res::decode(r)?)),
            NfsOpnum4::ReadDir => Ok(NfsResOp4::ReadDir(ReadDir4Res::decode(r)?)),
            // NfsOpnum4::ReadLink => Ok(NfsResOp4::ReadLink(ReadLink4Res::decode(r)?)),
            // NfsOpnum4::Remove => Ok(NfsResOp4::Remove(Remove4Res::decode(r)?)),
            // NfsOpnum4::Rename => Ok(NfsResOp4::Rename(Rename4Res::decode(r)?)),
            // NfsOpnum4::Renew => Ok(NfsResOp4::Renew(Renew4Res::decode(r)?)),
            // NfsOpnum4::RestoreFh => Ok(NfsResOp4::RestoreFh(RestoreFh4Res::decode(r)?)),
            // NfsOpnum4::SaveFh => Ok(NfsResOp4::SaveFh(SaveFh4Res::decode(r)?)),
            // NfsOpnum4::SecInfo => Ok(NfsResOp4::SecInfo(SecInfo4Res::decode(r)?)),
            // NfsOpnum4::SetAttr => Ok(NfsResOp4::SetAttr(SetAttr4Res::decode(r)?)),
            // NfsOpnum4::SetClientId => Ok(NfsResOp4::SetClientId(SetClientId4Res::decode(r)?)),
            // NfsOpnum4::SetClientIdConfirm => Ok(NfsResOp4::SetClientIdConfirm(SetClientIdConfirm4Res::decode(r)?)),
            // NfsOpnum4::Verify => Ok(NfsResOp4::Verify(Verify4Res::decode(r)?)),
            // NfsOpnum4::Write => Ok(NfsResOp4::Write(Write4Res::decode(r)?)),
            // NfsOpnum4::ReleaseLockOwner => Ok(NfsResOp4::ReleaseLockOwner(ReleaseLockOwner4Res::decode(r)?)),

            // NFSv4.1
            // NfsOpnum4::BackchannelCtl => Ok(NfsResOp4::BackchannelCtl(BackchannelCtl4Res::decode(r)?)),
            // NfsOpnum4::BindConnToSession => Ok(NfsResOp4::BindConnToSession(BindConnToSession4Res::decode(r)?)),
            NfsOpnum4::ExchangeId => Ok(NfsResOp4::ExchangeId(ExchangeId4Res::decode(r)?)),
            NfsOpnum4::CreateSession => Ok(NfsResOp4::CreateSession(CreateSession4Res::decode(r)?)),
            // NfsOpnum4::DestroySession => Ok(NfsResOp4::DestroySession(DestroySession4Res::decode(r)?)),
            // NfsOpnum4::FreeStateId => Ok(NfsResOp4::FreeStateId(FreeStateId4Res::decode(r)?)),
            // NfsOpnum4::GetDirDelegation => Ok(NfsResOp4::GetDirDelegation(GetDirDelegation4Res::decode(r)?)),
            // NfsOpnum4::GetDeviceInfo => Ok(NfsResOp4::GetDeviceInfo(GetDeviceInfo4Res::decode(r)?)),
            // NfsOpnum4::GetDeviceList => Ok(NfsResOp4::GetDeviceList(GetDeviceList4Res::decode(r)?)),
            // NfsOpnum4::LayoutCommit => Ok(NfsResOp4::LayoutCommit(LayoutCommit4Res::decode(r)?)),
            // NfsOpnum4::LayoutGet => Ok(NfsResOp4::LayoutGet(LayoutGet4Res::decode(r)?)),
            // NfsOpnum4::LayoutReturn => Ok(NfsResOp4::LayoutReturn(LayoutReturn4Res::decode(r)?)),
            // NfsOpnum4::SecInfoNoName => Ok(NfsResOp4::SecInfoNoName(SecInfoNoName4Res::decode(r)?)),
            NfsOpnum4::Sequence => Ok(NfsResOp4::Sequence(Sequence4Res::decode(r)?)),
            // NfsOpnum4::SetSsv => Ok(NfsResOp4::SetSsv(SetSsv4Res::decode(r)?)),
            // NfsOpnum4::TestStateId => Ok(NfsResOp4::TestStateId(TestStateId4Res::decode(r)?)),
            // NfsOpnum4::WantDelegation => Ok(NfsResOp4::WantDelegation(WantDelegation4Res::decode(r)?)),
            // NfsOpnum4::DestroyClientId => Ok(NfsResOp4::DestroyClientId(DestroyClientId4Res::decode(r)?)),
            // NfsOpnum4::ReclaimComplete => Ok(NfsResOp4::ReclaimComplete(ReclaimComplete4Res::decode(r)?)),
            NfsOpnum4::Illegal => Ok(NfsResOp4::Illegal(Illegal4Res::decode(r)?)),
            _ => Err(Nfsv4Error::NotImplementedOp(op)),
        }
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        match self {
            // NfsResOp4::Access(res) => {
            //     w.write_i32(NfsOpnum4::Access as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Close(res) => {
            //     w.write_i32(NfsOpnum4::Close as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Commit(res) => {
            //     w.write_i32(NfsOpnum4::Commit as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Create(res) => {
            //     w.write_i32(NfsOpnum4::Create as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::DelegPurge(res) => {
            //     w.write_i32(NfsOpnum4::DelegPurge as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::DelegReturn(res) => {
            //     w.write_i32(NfsOpnum4::DelegReturn as i32)?;
            //     res.encode(w)?;
            // }
            NfsResOp4::GetAttr(res) => {
                w.write_i32(NfsOpnum4::GetAttr as i32)?;
                res.encode(w)?;
            }
            // NfsResOp4::GetFh(res) => {
            //     w.write_i32(NfsOpnum4::GetFh as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Link(res) => {
            //     w.write_i32(NfsOpnum4::Link as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Lock(res) => {
            //     w.write_i32(NfsOpnum4::Lock as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::LockT(res) => {
            //     w.write_i32(NfsOpnum4::LockT as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::LockU(res) => {
            //     w.write_i32(NfsOpnum4::LockU as i32)?;
            //     res.encode(w)?;
            // }
            NfsResOp4::Lookup(res) => {
                w.write_i32(NfsOpnum4::Lookup as i32)?;
                res.encode(w)?;
            }
            // NfsResOp4::LookupP(res) => {
            //     w.write_i32(NfsOpnum4::LookupP as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::NVerify(res) => {
            //     w.write_i32(NfsOpnum4::NVerify as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Open(res) => {
            //     w.write_i32(NfsOpnum4::Open as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::OpenAttr(res) => {
            //     w.write_i32(NfsOpnum4::OpenAttr as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::OpenConfirm(res) => {
            //     w.write_i32(NfsOpnum4::OpenConfirm as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::OpenDowngrade(res) => {
            //     w.write_i32(NfsOpnum4::OpenDowngrade as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::PutFh(res) => {
            //     w.write_i32(NfsOpnum4::PutFh as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::PutPubFh(res) => {
            //     w.write_i32(NfsOpnum4::PutPubFh as i32)?;
            //     res.encode(w)?;
            // }
            NfsResOp4::PutRootFh(res) => {
                w.write_i32(NfsOpnum4::PutRootFh as i32)?;
                res.encode(w)?;
            }
            // NfsResOp4::Read(res) => {
            //     w.write_i32(NfsOpnum4::Read as i32)?;
            //     res.encode(w)?;
            // }
            NfsResOp4::ReadDir(res) => {
                w.write_i32(NfsOpnum4::ReadDir as i32)?;
                res.encode(w)?;
            }
            // NfsResOp4::ReadLink(res) => {
            //     w.write_i32(NfsOpnum4::ReadLink as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Remove(res) => {
            //     w.write_i32(NfsOpnum4::Remove as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Rename(res) => {
            //     w.write_i32(NfsOpnum4::Rename as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Renew(res) => {
            //     w.write_i32(NfsOpnum4::Renew as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::RestoreFh(res) => {
            //     w.write_i32(NfsOpnum4::RestoreFh as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::SaveFh(res) => {
            //     w.write_i32(NfsOpnum4::SaveFh as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::SecInfo(res) => {
            //     w.write_i32(NfsOpnum4::SecInfo as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::SetAttr(res) => {
            //     w.write_i32(NfsOpnum4::SetAttr as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::SetClientId(res) => {
            //     w.write_i32(NfsOpnum4::SetClientId as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::SetClientIdConfirm(res) => {
            //     w.write_i32(NfsOpnum4::SetClientIdConfirm as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Verify(res) => {
            //     w.write_i32(NfsOpnum4::Verify as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::Write(res) => {
            //     w.write_i32(NfsOpnum4::Write as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::ReleaseLockOwner(res) => {
            //     w.write_i32(NfsOpnum4::ReleaseLockOwner as i32)?;
            //     res.encode(w)?;
            // }

            // NFSv4.1
            // NfsResOp4::BackchannelCtl(res) => {
            //     w.write_i32(NfsOpnum4::BackchannelCtl as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::BindConnToSession(res) => {
            //     w.write_i32(NfsOpnum4::BindConnToSession as i32)?;
            //     res.encode(w)?;
            // }
            NfsResOp4::ExchangeId(res) => {
                w.write_i32(NfsOpnum4::ExchangeId as i32)?;
                res.encode(w)?;
            }
            NfsResOp4::CreateSession(res) => {
                w.write_i32(NfsOpnum4::CreateSession as i32)?;
                res.encode(w)?;
            }
            // NfsResOp4::DestroySession(res) => {
            //     w.write_i32(NfsOpnum4::DestroySession as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::FreeStateId(res) => {
            //     w.write_i32(NfsOpnum4::FreeStateId as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::GetDirDelegation(res) => {
            //     w.write_i32(NfsOpnum4::GetDirDelegation as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::GetDeviceInfo(res) => {
            //     w.write_i32(NfsOpnum4::GetDeviceInfo as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::GetDeviceList(res) => {
            //     w.write_i32(NfsOpnum4::GetDeviceList as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::LayoutCommit(res) => {
            //     w.write_i32(NfsOpnum4::LayoutCommit as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::LayoutGet(res) => {
            //     w.write_i32(NfsOpnum4::LayoutGet as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::LayoutReturn(res) => {
            //     w.write_i32(NfsOpnum4::LayoutReturn as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::SecInfoNoName(res) => {
            //     w.write_i32(NfsOpnum4::SecInfoNoName as i32)?;
            //     res.encode(w)?;
            // }
            NfsResOp4::Sequence(res) => {
                w.write_i32(NfsOpnum4::Sequence as i32)?;
                res.encode(w)?;
            }
            // NfsResOp4::SetSsv(res) => {
            //     w.write_i32(NfsOpnum4::SetSsv as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::TestStateId(res) => {
            //     w.write_i32(NfsOpnum4::TestStateId as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::WantDelegation(res) => {
            //     w.write_i32(NfsOpnum4::WantDelegation as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::DestroyClientId(res) => {
            //     w.write_i32(NfsOpnum4::DestroyClientId as i32)?;
            //     res.encode(w)?;
            // }
            // NfsResOp4::ReclaimComplete(res) => {
            //     w.write_i32(NfsOpnum4::ReclaimComplete as i32)?;
            //     res.encode(w)?;
            // }
            NfsResOp4::Illegal(res) => {
                w.write_i32(NfsOpnum4::Illegal as i32)?;
                res.encode(w)?;
            }
            _ => return Err(Nfsv4Error::InvalidEnumValue(NfsOpnum4::Illegal as i32)),
        }
        Ok(())
    }
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

impl Compound4Args {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            tag: r.read_string()?,
            minorversion: r.read_u32()?,
            argarray: read_array_nfs(r, |r| NfsArgOp4::decode(r))?,
        })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_string(&self.tag)?;
        w.write_u32(self.minorversion)?;
        write_array_nfs(w, &self.argarray, |w, arg| arg.encode(w))?;
        Ok(())
    }
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

impl Compound4Res {
    pub fn decode(r: &mut XdrReader) -> Result<Self, Nfsv4Error> {
        Ok(Self {
            status: Stat4::try_from(r.read_i32()?)?,
            tag: r.read_string()?,
            resarray: read_array_nfs(r, |r| NfsResOp4::decode(r))?,
        })
    }
    pub fn encode(&self, w: &mut XdrWriter) -> Result<(), Nfsv4Error> {
        w.write_i32(self.status as i32)?;
        w.write_string(&self.tag)?;
        write_array_nfs(w, &self.resarray, |w, res| res.encode(w))?;
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nfsv4::ops::Illegal4Res;
    use crate::nfsv4::types::Stat4;
    use xdr_rs::reader::XdrReader;
    use xdr_rs::writer::XdrWriter;

    #[test]
    fn nfs_opnum4_try_from_known_values() {
        assert_eq!(NfsOpnum4::try_from(24).unwrap(), NfsOpnum4::PutRootFh);
        assert_eq!(NfsOpnum4::try_from(26).unwrap(), NfsOpnum4::ReadDir);
        assert_eq!(NfsOpnum4::try_from(33).unwrap(), NfsOpnum4::SecInfo);
        assert_eq!(NfsOpnum4::try_from(40).unwrap(), NfsOpnum4::BackchannelCtl);
        assert_eq!(NfsOpnum4::try_from(42).unwrap(), NfsOpnum4::ExchangeId);
        assert_eq!(NfsOpnum4::try_from(43).unwrap(), NfsOpnum4::CreateSession);
        assert_eq!(NfsOpnum4::try_from(53).unwrap(), NfsOpnum4::Sequence);
        assert_eq!(NfsOpnum4::try_from(10044).unwrap(), NfsOpnum4::Illegal);
    }

    #[test]
    fn nfs_opnum4_try_from_unknown_value_returns_error() {
        let err = NfsOpnum4::try_from(9999).unwrap_err();
        match err {
            Nfsv4Error::InvalidNfsOpnum4(9999) => {}
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn nfs_argop4_decode_putrootfh() {
        let mut w = XdrWriter::new();
        w.write_i32(NfsOpnum4::PutRootFh as i32).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let op = NfsArgOp4::decode(&mut r).unwrap();

        assert_eq!(op, NfsArgOp4::PutRootFh);
    }

    #[test]
    fn nfs_argop4_encode_putrootfh() {
        let mut w = XdrWriter::new();
        NfsArgOp4::PutRootFh.encode(&mut w).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let raw = r.read_i32().unwrap();

        assert_eq!(raw, NfsOpnum4::PutRootFh as i32);
    }

    #[test]
    fn nfs_argop4_decode_unimplemented_op_returns_not_implemented() {
        let mut w = XdrWriter::new();
        w.write_i32(NfsOpnum4::GetAttr as i32).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let err = NfsArgOp4::decode(&mut r).unwrap_err();

        match err {
            Nfsv4Error::NotImplementedOp(NfsOpnum4::GetAttr) => {}
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn nfs_resop4_decode_illegal() {
        let mut w = XdrWriter::new();
        w.write_i32(NfsOpnum4::Illegal as i32).unwrap();
        w.write_i32(Stat4::OpIllegal as i32).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let op = NfsResOp4::decode(&mut r).unwrap();

        assert_eq!(
            op,
            NfsResOp4::Illegal(Illegal4Res {
                status: Stat4::OpIllegal,
            })
        );
    }

    #[test]
    fn nfs_resop4_encode_illegal() {
        let op = NfsResOp4::Illegal(Illegal4Res {
            status: Stat4::OpIllegal,
        });

        let mut w = XdrWriter::new();
        op.encode(&mut w).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let raw_op = r.read_i32().unwrap();
        let raw_status = r.read_i32().unwrap();

        assert_eq!(raw_op, NfsOpnum4::Illegal as i32);
        assert_eq!(raw_status, Stat4::OpIllegal as i32);
    }

    #[test]
    fn nfs_resop4_decode_unimplemented_op_returns_not_implemented() {
        let mut w = XdrWriter::new();
        w.write_i32(NfsOpnum4::Access as i32).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let err = NfsResOp4::decode(&mut r).unwrap_err();

        match err {
            Nfsv4Error::NotImplementedOp(NfsOpnum4::Access) => {}
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn compound4args_roundtrip_with_supported_ops() {
        let original = Compound4Args {
            tag: "test-tag".to_string(),
            minorversion: 1,
            argarray: vec![NfsArgOp4::PutRootFh],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let decoded = Compound4Args::decode(&mut r).unwrap();

        assert_eq!(decoded, original);
    }

    #[test]
    fn compound4res_roundtrip_with_illegal() {
        let original = Compound4Res {
            status: Stat4::OpIllegal,
            tag: "illegal".to_string(),
            resarray: vec![NfsResOp4::Illegal(Illegal4Res {
                status: Stat4::OpIllegal,
            })],
        };

        let mut w = XdrWriter::new();
        original.encode(&mut w).unwrap();

        let mut r = XdrReader::new(&w.as_bytes());
        let decoded = Compound4Res::decode(&mut r).unwrap();

        assert_eq!(decoded, original);
    }
}

#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use xdr_rs::reader::XdrReader;
use xdr_rs::writer::XdrWriter;

use crate::error::Nfsv4Error;
use crate::nfsv4::types::*;

/// RFC7531: createtype4
///
/// Type-specific data used by the CREATE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateType4 {
    /// Symbolic link with link target data.
    SymbolicLink(LinkText4),

    /// Block device special file.
    BlockDevice(SpecData4),

    /// Character device special file.
    CharacterDevice(SpecData4),

    /// Socket special file.
    Socket,

    /// FIFO special file.
    Fifo,

    /// Directory.
    Directory,
}

/// RFC7531: CREATE4args
///
/// Arguments for the CREATE operation.
/// The current filehandle (CURRENT_FH) must refer to a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Create4Args {
    /// Type of object to create (file, directory, symlink, device, etc.).
    pub objtype: CreateType4,

    /// Name of the object to create.
    pub objname: Component4,

    /// Attributes to set during creation.
    pub createattrs: Fattr4,
}

/// RFC7531: CREATE4resok
///
/// Successful result of the CREATE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Create4ResOk {
    /// Change information for the directory.
    pub cinfo: ChangeInfo4,

    /// Bitmap indicating which attributes were set.
    pub attrset: Bitmap4,
}

/// RFC7531: CREATE4res
///
/// Result of the CREATE operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Create4Res {
    /// Operation succeeded.
    Ok(Create4ResOk),

    /// Operation failed with an NFS error status.
    Err(Stat4),
}

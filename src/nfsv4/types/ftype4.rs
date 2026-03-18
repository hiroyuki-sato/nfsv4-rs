#![allow(dead_code)]

use crate::error::Nfsv4Error;

/// RFC7531: nfs_ftype4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Ftype4 {
    /// NF4REG - Regular File
    RegularFile = 1,

    /// NF4DIR - Directory
    Directory = 2,

    /// NF4BLK - Special File - block device
    BlockDevice = 3,

    /// NF4CHR - Special File - character device
    CharacterDevice = 4,

    /// NF4LNK - Symbolic Link
    SymbolicLink = 5,

    /// NF4SOCK - Special File - socket
    Socket = 6,

    /// NF4FIFO - Special File - fifo
    Fifo = 7,

    /// NF4ATTRDIR - Attribute Directory
    AttributeDirectory = 8,

    /// NF4NAMEDATTR - Named Attribute
    NamedAttribute = 9,
}

impl TryFrom<i32> for Ftype4 {
    type Error = Nfsv4Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::RegularFile),
            2 => Ok(Self::Directory),
            3 => Ok(Self::BlockDevice),
            4 => Ok(Self::CharacterDevice),
            5 => Ok(Self::SymbolicLink),
            6 => Ok(Self::Socket),
            7 => Ok(Self::Fifo),
            8 => Ok(Self::AttributeDirectory),
            9 => Ok(Self::NamedAttribute),
            _ => Err(Nfsv4Error::InvalidNfsFiletype(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftype4_try_from_valid() {
        assert!(matches!(Ftype4::try_from(1).unwrap(), Ftype4::RegularFile));
        assert!(matches!(Ftype4::try_from(2).unwrap(), Ftype4::Directory));
        assert!(matches!(Ftype4::try_from(3).unwrap(), Ftype4::BlockDevice));
        assert!(matches!(
            Ftype4::try_from(4).unwrap(),
            Ftype4::CharacterDevice
        ));
        assert!(matches!(Ftype4::try_from(5).unwrap(), Ftype4::SymbolicLink));
        assert!(matches!(Ftype4::try_from(6).unwrap(), Ftype4::Socket));
        assert!(matches!(Ftype4::try_from(7).unwrap(), Ftype4::Fifo));
        assert!(matches!(
            Ftype4::try_from(8).unwrap(),
            Ftype4::AttributeDirectory
        ));
        assert!(matches!(
            Ftype4::try_from(9).unwrap(),
            Ftype4::NamedAttribute
        ));
    }

    #[test]
    fn test_ftype4_try_from_invalid() {
        let err = Ftype4::try_from(999).unwrap_err();
        assert!(matches!(err, Nfsv4Error::InvalidNfsFiletype(999)));
    }
}

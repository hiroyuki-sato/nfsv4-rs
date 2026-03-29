#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nfsv4Version {
    V4_0 = 0,
    V4_1 = 1,
    V4_2 = 2,
}

impl TryFrom<u32> for Nfsv4Version {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::V4_0),
            1 => Ok(Self::V4_1),
            2 => Ok(Self::V4_2),
            _ => Err(()),
        }
    }
}

impl Nfsv4Version {
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfsv4version_try_from_valid() {
        assert_eq!(Nfsv4Version::try_from(0), Ok(Nfsv4Version::V4_0));
        assert_eq!(Nfsv4Version::try_from(1), Ok(Nfsv4Version::V4_1));
        assert_eq!(Nfsv4Version::try_from(2), Ok(Nfsv4Version::V4_2));
    }

    #[test]
    fn test_nfsv4version_try_from_invalid() {
        assert_eq!(Nfsv4Version::try_from(3), Err(()));
        assert_eq!(Nfsv4Version::try_from(u32::MAX), Err(()));
    }

    #[test]
    fn test_nfsv4version_as_u32() {
        assert_eq!(Nfsv4Version::V4_0.as_u32(), 0);
        assert_eq!(Nfsv4Version::V4_1.as_u32(), 1);
        assert_eq!(Nfsv4Version::V4_2.as_u32(), 2);
    }

    #[test]
    fn test_nfsv4version_roundtrip() {
        for v in [Nfsv4Version::V4_0, Nfsv4Version::V4_1, Nfsv4Version::V4_2] {
            assert_eq!(Nfsv4Version::try_from(v.as_u32()), Ok(v));
        }
    }
}

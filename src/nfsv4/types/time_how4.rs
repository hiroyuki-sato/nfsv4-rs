#![allow(dead_code)]

use super::*;

/// RFC7531: time_how4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TimeHow4 {
    SetToServerTime = 0,
    SetToClientTime = 1,
}

impl TryFrom<i32> for TimeHow4 {
    type Error = Nfsv4Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TimeHow4::SetToServerTime),
            1 => Ok(TimeHow4::SetToClientTime),
            _ => Err(Nfsv4Error::InvalidTimeHow4(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timehow4_try_from_valid() {
        assert!(matches!(
            TimeHow4::try_from(0).unwrap(),
            TimeHow4::SetToServerTime
        ));

        assert!(matches!(
            TimeHow4::try_from(1).unwrap(),
            TimeHow4::SetToClientTime
        ));
    }

    #[test]
    fn test_timehow4_try_from_invalid() {
        let err = TimeHow4::try_from(42).unwrap_err();

        assert!(matches!(err, Nfsv4Error::InvalidTimeHow4(42)));
    }
}

#![allow(dead_code)]

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// Mode4 bitmask values (fattr4_mode attribute)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mode4(u32);

impl Mode4 {
    pub const SET_USER_ID: Self = Self(0x800); // set user id on execution
    pub const SET_GROUP_ID: Self = Self(0x400); // set group id on execution
    pub const SAVE_TEXT: Self = Self(0x200); // save text even after use

    pub const READ_OWNER: Self = Self(0x100); // read permission: owner
    pub const WRITE_OWNER: Self = Self(0x080); // write permission: owner
    pub const EXEC_OWNER: Self = Self(0x040); // execute permission: owner

    pub const READ_GROUP: Self = Self(0x020); // read permission: group
    pub const WRITE_GROUP: Self = Self(0x010); // write permission: group
    pub const EXEC_GROUP: Self = Self(0x008); // execute permission: group

    pub const READ_OTHER: Self = Self(0x004); // read permission: other
    pub const WRITE_OTHER: Self = Self(0x002); // write permission: other
    pub const EXEC_OTHER: Self = Self(0x001); // execute permission: other

    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub fn empty() -> Self {
        Self(0)
    }
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn bits(self) -> u32 {
        self.0
    }

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
}

impl BitOr for Mode4 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Mode4 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Mode4 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Mode4 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for Mode4 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<u32> for Mode4 {
    fn from(bits: u32) -> Self {
        Self(bits)
    }
}

impl From<Mode4> for u32 {
    fn from(mode: Mode4) -> Self {
        mode.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_mode() {
        let mode = Mode4::empty();
        assert!(mode.is_empty());
        assert_eq!(mode.bits(), 0);
    }

    #[test]
    fn test_from_bits_and_into_u32() {
        let mode = Mode4::from_bits(0x123);
        assert_eq!(mode.bits(), 0x123);

        let bits: u32 = mode.into();
        assert_eq!(bits, 0x123);
    }

    #[test]
    fn test_from_u32() {
        let mode = Mode4::from(0x1ff);
        assert_eq!(mode.bits(), 0x1ff);
    }

    #[test]
    fn test_contains_single_flag() {
        let mode = Mode4::READ_OWNER | Mode4::WRITE_OWNER;
        assert!(mode.contains(Mode4::READ_OWNER));
        assert!(mode.contains(Mode4::WRITE_OWNER));
        assert!(!mode.contains(Mode4::EXEC_OWNER));
    }

    #[test]
    fn test_contains_multiple_flags() {
        let mode = Mode4::READ_OWNER | Mode4::WRITE_OWNER | Mode4::READ_GROUP;
        assert!(mode.contains(Mode4::READ_OWNER | Mode4::WRITE_OWNER));
        assert!(!mode.contains(Mode4::READ_OWNER | Mode4::EXEC_OWNER));
    }

    #[test]
    fn test_insert_flag() {
        let mut mode = Mode4::empty();
        mode.insert(Mode4::READ_OWNER);
        assert!(mode.contains(Mode4::READ_OWNER));
        assert_eq!(mode.bits(), 0x100);

        mode.insert(Mode4::WRITE_OWNER);
        assert!(mode.contains(Mode4::WRITE_OWNER));
        assert_eq!(mode.bits(), 0x180);
    }

    #[test]
    fn test_remove_flag() {
        let mut mode = Mode4::READ_OWNER | Mode4::WRITE_OWNER | Mode4::EXEC_OWNER;
        mode.remove(Mode4::WRITE_OWNER);

        assert!(mode.contains(Mode4::READ_OWNER));
        assert!(!mode.contains(Mode4::WRITE_OWNER));
        assert!(mode.contains(Mode4::EXEC_OWNER));
        assert_eq!(mode.bits(), 0x140);
    }

    #[test]
    fn test_set_true() {
        let mut mode = Mode4::empty();
        mode.set(Mode4::READ_OWNER, true);

        assert!(mode.contains(Mode4::READ_OWNER));
        assert_eq!(mode.bits(), 0x100);
    }

    #[test]
    fn test_set_false() {
        let mut mode = Mode4::READ_OWNER | Mode4::WRITE_OWNER;
        mode.set(Mode4::WRITE_OWNER, false);

        assert!(mode.contains(Mode4::READ_OWNER));
        assert!(!mode.contains(Mode4::WRITE_OWNER));
        assert_eq!(mode.bits(), 0x100);
    }

    #[test]
    fn test_bitor() {
        let mode = Mode4::READ_OWNER | Mode4::WRITE_OWNER | Mode4::EXEC_OWNER;
        assert_eq!(mode.bits(), 0x1c0);
    }

    #[test]
    fn test_bitor_assign() {
        let mut mode = Mode4::READ_OWNER;
        mode |= Mode4::WRITE_OWNER;
        mode |= Mode4::EXEC_OWNER;

        assert_eq!(mode.bits(), 0x1c0);
    }

    #[test]
    fn test_bitand() {
        let mode = Mode4::READ_OWNER | Mode4::WRITE_OWNER | Mode4::EXEC_OWNER;
        let masked = mode & (Mode4::READ_OWNER | Mode4::EXEC_OWNER);

        assert_eq!(masked.bits(), 0x140);
        assert!(masked.contains(Mode4::READ_OWNER));
        assert!(masked.contains(Mode4::EXEC_OWNER));
        assert!(!masked.contains(Mode4::WRITE_OWNER));
    }

    #[test]
    fn test_bitand_assign() {
        let mut mode = Mode4::READ_OWNER | Mode4::WRITE_OWNER | Mode4::EXEC_OWNER;
        mode &= Mode4::READ_OWNER | Mode4::EXEC_OWNER;

        assert_eq!(mode.bits(), 0x140);
    }

    #[test]
    fn test_not() {
        let mode = !Mode4::empty();
        assert_eq!(mode.bits(), !0u32);
    }

    #[test]
    fn test_common_unix_mode_0644() {
        let mode = Mode4::READ_OWNER | Mode4::WRITE_OWNER | Mode4::READ_GROUP | Mode4::READ_OTHER;

        assert_eq!(mode.bits(), 0o644);
    }

    #[test]
    fn test_common_unix_mode_0755() {
        let mode = Mode4::READ_OWNER
            | Mode4::WRITE_OWNER
            | Mode4::EXEC_OWNER
            | Mode4::READ_GROUP
            | Mode4::EXEC_GROUP
            | Mode4::READ_OTHER
            | Mode4::EXEC_OTHER;

        assert_eq!(mode.bits(), 0o755);
    }
}

#![allow(dead_code)]

use super::*;

/// RFC7531: fsid4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fsid4 {
    pub major: u64,
    pub minor: u64,
}

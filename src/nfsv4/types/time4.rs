#![allow(dead_code)]

use super::*;

/// RFC7531: nfstime4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NfsTime4 {
    pub seconds: i64,
    pub nseconds: u32,
}

#![allow(dead_code)]

use super::*;

/// RFC7531: settime4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetTime4 {
    SetToServerTime,
    SetToClientTime(NfsTime4),
}

#![allow(dead_code)]

use super::*;

/// RFC7531: time_how4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TimeHow4 {
    SetToServerTime = 0,
    SetToClientTime = 1,
}

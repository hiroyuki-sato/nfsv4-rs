#![allow(dead_code)]

use super::*;

/// RFC7531: fs_location4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsLocation4 {
    pub server: Vec<String>,
    pub rootpath: Pathname4,
}

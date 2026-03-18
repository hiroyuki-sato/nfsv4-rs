#![allow(dead_code)]

use super::*;

/// RFC7531: fs_locations4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsLocations4 {
    pub fs_root: Pathname4,
    pub locations: Vec<FsLocation4>,
}

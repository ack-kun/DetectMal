use crate::hdb;
use std::collections::BTreeSet;
use std::path::Path;

pub struct HashDBMatcher {
    list: BTreeSet<[u8; 16]>,
}

impl HashDBMatcher {
    pub fn new() -> HashDBMatcher {
        HashDBMatcher {
            list: hdb::load("whitelist.hdb"),
        }
    }

    pub fn check(&self, hash: &[u8; 16]) -> bool {
        self.list.contains(hash)
    }
}

pub fn check_extension(path: &Path) -> bool {
    path.extension() == None || path.extension().unwrap() != "php"
}

use crate::hdb;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use std::collections::BTreeSet;

pub struct HashDBMatcher {
    list: BTreeSet<[u8; 16]>,
}

impl HashDBMatcher {
    pub fn new() -> HashDBMatcher {
        HashDBMatcher {
            list: hdb::load("blacklist.hdb"),
        }
    }

    pub fn check(&self, hash: &[u8; 16]) -> bool {
        self.list.contains(hash)
    }
}

pub struct Substring {
    matcher: AhoCorasick,
}

impl<'a> Substring {
    pub fn new(map: &Vec<String>) -> Substring {
        let matcher = AhoCorasickBuilder::new()
            .ascii_case_insensitive(true)
            .dfa(true)
            .build(map);

        Substring { matcher }
    }

    pub fn check(&self, content: &[u8]) -> bool {
        self.matcher.is_match(content)
    }
}

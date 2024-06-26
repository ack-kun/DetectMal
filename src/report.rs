use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScanReport {
    pub scan_path: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub directory_found: u64,
    pub files_found: u64,
    pub extension_skips: u64,
    pub whitelist_hits: u64,
    pub blacklist_hits: u64,
    pub substring_hits: u64,
}

impl ScanReport {
    pub fn new(scan_path: String) -> ScanReport {
        ScanReport {
            scan_path,
            started_at: Utc::now().to_rfc3339(),
            finished_at: None,
            directory_found: 0,
            files_found: 0,
            extension_skips: 0,
            whitelist_hits: 0,
            blacklist_hits: 0,
            substring_hits: 0,
        }
    }

    pub fn file_hit(&mut self) {
        self.files_found += 1;
    }

    pub fn directory_hit(&mut self) {
        self.directory_found += 1;
    }

    pub fn whitelist_hit(&mut self) {
        self.whitelist_hits += 1;
    }

    pub fn blacklist_hit(&mut self) {
        self.blacklist_hits += 1;
    }

    pub fn extension_skip(&mut self) {
        self.extension_skips += 1;
    }

    pub fn substring_hit(&mut self) {
        self.substring_hits += 1;
    }

    pub fn finished(&mut self) {
        self.finished_at = Some(Utc::now().to_rfc3339());
    }

    pub fn to_json(self) -> String {
        return serde_json::to_string_pretty(&self).expect("Could not serialize the scan result");
    }
}

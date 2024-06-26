use md5::compute;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use took::Timer;
use walkdir::WalkDir;

use crate::csv;
use crate::detectors::{blacklist, character, whitelist};
use crate::report::ScanReport;

pub fn run(scan_path: &str) {
    let timer = Timer::new();

    let mut report = ScanReport::new(scan_path.to_string());

    println!("Using scan path: {}", scan_path);

    // Load databases
    let blacklist_hdb = blacklist::HashDBMatcher::new();
    let whitelist_hdb = whitelist::HashDBMatcher::new();

    // Load suspicious malware substrings.
    let sus_pat = csv::load("suspicious.csv");
    let sus_matcher = blacklist::Substring::new(&sus_pat);

    // Load exact malware substrings.
    let unq_pat = csv::load("uniques.csv");
    let unq_matcher = blacklist::Substring::new(&unq_pat);

    for entry in WalkDir::new(scan_path).follow_links(false).max_depth(512) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            report.directory_hit();
            continue;
        }

        report.file_hit();

        if report.files_found % 1000 == 0 {
            println!("File scanned {}", report.files_found);
        }

        // Check if the extension PHP
        if whitelist::check_extension(&path) {
            report.extension_skip();
            continue;
        }

        let content = read_file(&path);
        let hash = compute(&content);

        // Hash the values for normalized md5

        // Bucket by length, and then build distribution rate 10k a=35% !=1%
        // store those values to find the malwares

        if whitelist_hdb.check(&hash.0) {
            report.whitelist_hit();
            continue;
        }

        if blacklist_hdb.check(&hash.0) {
            report.blacklist_hit();
            println!("Blacklist hit {:x} ->> {}", hash, path.display());

            continue;
        }

        if unq_matcher.check(&content) {
            report.substring_hit();
            println!("Direct substring match: in {}", path.display());

            continue;
        }

        // Normalize the content into logical bytes.
        let normalized = character::normalize(&content);
        let n_hash = md5::compute(&normalized);

        // println!("Normalized form:\n{:?}", String::from_utf8(normalized));
        println!("Normalized hash: {:x}", n_hash);

        // Match substrings which are used in obfuscation / malicious code.
        if sus_matcher.check(&content) {
            println!("Suspicious substring match: in {}", path.display());
        }
    }

    println!(
        "--------------------------\nFinished! Elapsed {}\n--------------------------",
        timer.took()
    );

    report.finished();

    println!("Scan report {}", report.to_json());
}

fn read_file(path: &Path) -> Vec<u8> {
    let mut f = File::open(path).expect("Could not open the file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Read failed");

    return buffer;
}

use md5::compute;
use std::collections::BTreeSet;
use std::env::current_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use took::Timer;
use walkdir::WalkDir;

use crate::detectors::whitelist;

pub fn run(sample_path: &str, mode: &str) {
    let timer = Timer::new();
    println!("Using learn path: {}", sample_path);

    let mut count_files: u32 = 0;
    let mut db: BTreeSet<[u8; 16]> = BTreeSet::new();

    for entry in WalkDir::new(sample_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() || whitelist::check_extension(&path) {
            continue;
        }

        let mut f = File::open(path).expect("Could not open the file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("Read failed");

        let size = buffer.len() / 1024;
        let digest = compute(buffer);

        db.insert(digest.0);

        println!("Hash {:x} Path {} Size {} kB", digest, path.display(), size);

        count_files += 1;
    }

    println!("Finished! Elapsed {}", timer.took());
    println!(
        "Total files {}x Unique signatures {}x",
        count_files,
        db.len()
    );

    let curr_dir = current_dir().unwrap();

    let target_hdb = if mode == "black" {
        "blacklist.hdb"
    } else {
        "whitelist.hdb"
    };

    let mut hdb = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(curr_dir.as_path().join("databases").join(target_hdb))
        .expect("Could not open hash database file!");

    for item in db.iter() {
        hdb.write_all(item).expect("Could not write entry to HDB");
    }

    println!("Created database: {:?}", target_hdb);
}

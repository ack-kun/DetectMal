use std::collections::BTreeSet;
use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn load(mode: &str) -> BTreeSet<[u8; 16]> {
    let db_path = current_dir()
        .unwrap()
        .as_path()
        .join("databases")
        .join(mode);

    if !db_path.exists() {
        panic!("No database found in path {}", db_path.display());
    }

    // Measure the database, use the byte length / 16 to calculate the BTreeSet size, and load every 16 byte into a record.

    let mut database = BTreeSet::new();
    let mut reader = OpenOptions::new()
        .read(true)
        .open(&db_path)
        .expect("Could not read the HDB");

    let fmeta = reader.metadata().unwrap();
    let records = fmeta.len() / 16;

    for _x in 0..records {
        let mut buff: [u8; 16] = [0; 16];

        reader.read_exact(&mut buff).expect("Could not read chunk");
        database.insert(buff);
    }

    database
}

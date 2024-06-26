use csv::Reader;
use std::env::current_dir;

pub fn load(mode: &str) -> Vec<String> {
    let db_path = current_dir()
        .unwrap()
        .as_path()
        .join("databases")
        .join(mode);

    if !db_path.exists() {
        panic!("No database found in path {}", db_path.display());
    }

    let mut database = Vec::new();
    let mut reader = Reader::from_path(&db_path).expect("Could not read the CSV database");

    for record in reader.records() {
        let p = String::from(record.unwrap().as_slice());
        database.push(p);
    }

    database
}

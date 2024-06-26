use clap::{App, Arg, SubCommand};
mod commands;
mod csv;
mod detectors;
mod hdb;
mod report;

use commands::{learn, scan};

fn main() {
    let matches = App::new("Malware Detect")
        .version("0.1.0")
        .about("We love llamas!")
        .subcommand(
            SubCommand::with_name("scan")
                .about("Scan the path for malicious PHP files")
                .author("Zsolt Varga <hello@hisorange.me>")
                .arg(
                    Arg::with_name("PATH")
                        .help("Sets the scan path")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("learn")
                .about("Learn the directory of malwares")
                .author("Zsolt Varga <hello@hisorange.me>")
                .arg(
                    Arg::with_name("PATH")
                        .help("Sets the learn path")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("MODE")
                        .short("m")
                        .long("mode")
                        .required(true)
                        .takes_value(true)
                        .possible_values(&["black", "white"])
                        .help("Database type"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("scan", Some(matches)) => {
            scan::run(matches.value_of("PATH").unwrap());
        }
        ("learn", Some(matches)) => {
            learn::run(
                matches.value_of("PATH").unwrap(),
                matches.value_of("MODE").unwrap(),
            );
        }
        _ => {
            println!("{}", matches.usage());
        }
    }
}

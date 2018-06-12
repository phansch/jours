extern crate clap;
extern crate chrono;
use clap::{Arg, App};
use std::io::Write;
use std::fs::{OpenOptions};

fn main() {
    let matches = App::new("jours")
        .version("0.0.1")
        .author("Philipp Hansch <dev@phansch.net>")
        .about("Daily journals and note taking on the command line")
        .arg(Arg::with_name("add")
             .short("a")
             .long("add")
             .value_name("TEXT")
             .help("Add an entry")
             .takes_value(true))
        .get_matches();

    if let Some(to_add) = matches.value_of("add") {
        add_to_file(to_add);
    };
}

fn add_to_file(value: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("/tmp/foo").unwrap();
    let value = formatted_line(value);
    file.write_all(value.as_bytes()).unwrap();
}

fn formatted_line(value: &str) -> String {
    format!("* {}\n", value)
}

#[test]
fn test_formatted_line() {
    assert_eq!("* test\n", formatted_line("test"));
}

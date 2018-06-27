extern crate clap;
extern crate chrono;
extern crate config;
extern crate directories;

mod settings;

use std::io::Write;
use std::fs::{OpenOptions};
use chrono::prelude::*;
use clap::{Arg, App};
use settings::Settings;

struct NewEntry {
    value: String,
    datetime: DateTime<Local>
}

impl NewEntry {
    fn new(value: &str) -> NewEntry {
        NewEntry {
            value: value.to_string(),
            datetime: Local::now()
        }
    }

    fn formatted_line(&self) -> String {
        format!("* {}: {}\n", self.datetime.format("%X"), self.value)
    }
}

fn main() {
    let settings = Settings::new();
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
    let new_entry = NewEntry::new(value);
    let value = new_entry.formatted_line();
    file.write_all(value.as_bytes()).unwrap();
}

#[test]
fn test_formatted_line() {
    let now = Local::now().format("%X");
    let expected = format!("* {}: test\n", now);
    assert_eq!(expected, NewEntry::new("test").formatted_line());
}

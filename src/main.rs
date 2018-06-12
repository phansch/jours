extern crate clap;
use clap::{Arg, App};

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


    println!("Hello, world!");
}

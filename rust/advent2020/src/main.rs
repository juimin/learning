extern crate adventlib;
extern crate clap;
use clap::{App, Arg};

mod challenges;

fn main() {
    let args = App::new("Advent of Code 2020")
        .version("0.1")
        .author("Derek W.")
        .about("Command Line program for 2020's advent of code")
        .arg(Arg::with_name("day")
            .help("Day with which to run")
            .short("d")
            .long("day")
            .required(true)
            .value_name("DAY")
            .takes_value(true))
        .arg(Arg::with_name("file")
            .help("File path used for input")
            .short("f")
            .long("file")
            .required(true)
            .value_name("FILE")
            .takes_value(true)
    )
    .get_matches();

    let day: u8 = args.value_of("day").unwrap_or("0").trim().parse().expect("Day needs to be a number");
    let file = args.value_of("file").unwrap_or("");

    challenges::run(day, file);
}

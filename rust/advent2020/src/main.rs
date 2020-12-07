extern crate adventlib;
extern crate clap;
use clap::{App, Arg};

mod challenges;

fn main() {
    let args = App::new("Advent of Code 2020")
        .version("0.1")
        .author("Derek W.")
        .about("Command Line program for 2020's advent of code")
        .arg(
            Arg::with_name("day")
                .help("Day with which to run")
                .short("d")
                .long("day")
                .required(true)
                .value_name("DAY")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .help("File path used for input")
                .short("f")
                .long("file")
                .required(false)
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let day_str = args.value_of("day").unwrap_or("0");
    let day: u8 = day_str.trim().parse().expect("Day needs to be a number");
    let mut file = String::new();
    file.push_str(args.value_of("file").unwrap_or(""));

    if file.len() == 0 {
        // Expect the data directory in the same dir
        file.push_str(&format!("./advent2020inputs/day{}.txt", day_str))
    }

    challenges::run(day, &file);
}

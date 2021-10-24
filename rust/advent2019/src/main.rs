mod advent;
use clap::{App, Arg};

fn main() {
    let args = App::new("Advent of Code 2019")
        .version("0,1")
        .author("Derek W.")
        .about("CLI for 2019 Advent of Code")
        .arg(
            Arg::with_name("file")
                .help("Path to inputs for this challenge")
                .short("f")
                .required(true)
                .value_name("file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("day")
            .help("Day with which to run")
            .short("d")
            .required(true)
            .value_name("day")
            .takes_value(true),
        )
        .get_matches();
    
    // Parse the args to get the data we need
    let day: u8 = args.value_of("day").unwrap_or("0")
        .trim().parse().expect("Day needs to be a number");
    let file = args.value_of("file").unwrap_or("").to_string();

    // Log the desire of the user
    println!("Advent of Code 2019 Day {}: Processing {}\n", day, file);
    
    advent::run(day, file)
}

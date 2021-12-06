mod advent2019;
mod advent2021;
use clap::{App, Arg};

fn run_for_year(year: i64, day: u8, file: &String) {
    match year {
        2019 => {
            advent2019::run(day, file);
        }
        2021 => {
            advent2021::run(day, file);
        }
        _ => {
            println!("No implementation for year {}", year);
        }
    }
}

fn main() {
    let args = App::new("Advent of Code 2019")
        .version("0,1")
        .author("Derek W.")
        .about("CLI for Advent of Code")
        .arg(
            Arg::with_name("file")
                .help("Path to inputs for this challenge")
                .short("f")
                .required(false)
                .value_name("file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("day")
                .help("Day with which to run")
                .short("d")
                .required(true)
                .value_name("day")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("year")
                .help("Advent of Code year")
                .short("y")
                .required(true)
                .value_name("year")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test")
                .help("Whether or not to use test")
                .short("t")
                .required(false)
                .value_name("test")
                .takes_value(false),
        )
        .get_matches();
    // Parse the args to get the data we need
    let day: u8 = args
        .value_of("day")
        .unwrap_or("0")
        .trim()
        .parse()
        .expect("Day needs to be a number");
    let year: i64 = args
        .value_of("year")
        .unwrap_or("0")
        .trim()
        .parse()
        .expect("Day needs to be a number");
    let mut file = args.value_of("file").unwrap_or("").to_string();
    let testing = args.is_present("test");

    // Log the desire of the user
    println!("Advent of Code 2019 Day {}:", day);

    if file.len() == 0 {
        let mut dir = "input";
        if testing {
            dir = "test"
        }
        file = format!("./{}_data/{}/day{}.txt", dir, year, day);
    }

    println!("Processing: {}", &file);
    run_for_year(year, day, &file);
}

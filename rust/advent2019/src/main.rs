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
                .required(false)
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
        .arg(
            Arg::with_name("test")
            .help("Whether or not to use test")
            .short("t")
            .required(false)
            .value_name("test")
            .takes_value(false)
        )
        .get_matches();
    
    // Parse the args to get the data we need
    let day: u8 = args.value_of("day").unwrap_or("0")
        .trim().parse().expect("Day needs to be a number");
    let mut file = args.value_of("file").unwrap_or("").to_string();
    let testing = args.is_present("test");


    // Log the desire of the user
    println!("Advent of Code 2019 Day {}:", day);

    if file.len() == 0 {
        let mut dir = "./input_data/";
        if testing {
            dir = "./input_data/"
        }
        file = format!("{}day{}.txt", dir, day);
    }

    
    println!("Processing: {}", &file);
    advent::run(day, &file)
}

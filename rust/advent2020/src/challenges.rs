use adventlib;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub fn run(day: u8, file: &str) {
    if let Ok(lines) = adventlib::read_lines(file) {
        let results = match day {
            1 => day1::run(lines),
            2 => day2::run(lines),
            3 => day3::run(lines),
            4 => day4::run(lines),
            5 => day5::run(lines),
            6 => day6::run(lines),
            7 => day7::run(lines),
            8 => day8::run(lines),
            _ => (-1, -1),
        };
        println!("Day {} Part 1: {}", day, results.0);
        println!("Day {} Part 2: {}", day, results.1);
    }
}

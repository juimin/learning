// Include library modules
mod util;

// Add modules for each day
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("Advent of Code!");
    day1::run();
    day2::run();
    day3::run();
    day4::run();
    day5::run();
}
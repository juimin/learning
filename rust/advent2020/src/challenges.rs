mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;


pub fn run(day:u8, file: &str) {
    println!("Processing Challenge Day {}: {}\n", day, file);

    match day {
        1 => day1::run(file),
        2 => day2::run(file),
        3 => day3::run(file),
        4 => day4::run(file),
        5 => day5::run(file),
        6 => day6::run(file),
        _ => println!("The challenge for Day {} has not been completed", day)
    }
}
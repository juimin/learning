mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn run(day:u8, file: &str) {
    let file_contents = adventlib::file_contents_as_string(file);
    let results = match day {
        1 => day1::run(&file_contents),
        2 => day2::run(&file_contents),
        3 => {
            day3::run(&file_contents)
        },
        4 => day4::run(&file_contents),
        5 => day5::run(&file_contents),
        6 => day6::run(&file_contents),
        _ => (-1, -1)
    };
    println!("Day {} Part 1: {}", day, results.0);
    println!("Day {} Part 2: {}", day, results.1);
}
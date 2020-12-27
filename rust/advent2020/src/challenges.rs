use adventlib;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day24;

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
            9 => day9::run(lines),
            10 => day10::run(lines),
            11 => day11::run(lines),
            12 => day12::run(lines),
            // 13 => day13::run(lines),
            // 14 => day14::run(lines),
            // 15 => day15::run(lines),
            // 16 => day16::run(lines),
            // 17 => day17::run(lines),
            // 18 => day18::run(lines),
            // 19 => day19::run(lines),
            // 20 => day20::run(lines),
            // 21 => day21::run(lines),
            // 22 => day22::run(lines),
            // 23 => day23::run(lines),
            // 24 => day24::run(lines),
            // 25 => day25::run(lines),
            _ => (-1, -1),
        };
        println!("Day {} Part 1: {}", day, results.0);
        println!("Day {} Part 2: {}", day, results.1);
    }
}

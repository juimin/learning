use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn file_contents_as_string(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => return file_contents
    }
}

fn parse_contents(file_contents: &str) {
    // Get the valid password count
    let mut valid_total = 0;
    let mut valid_total_part_2 = 0;
    for line in file_contents.lines() {
        let mut splits = line.split_whitespace();
        // Extract the segments from the line
        let p1 = splits.next();
        let mut range_split = p1.unwrap().split("-");
        let p2 = splits.next();
        let target = p2.unwrap().chars().next().unwrap();
        let p3 = splits.next();
        let password = p3.unwrap();

        // Get the range
        let min: i32 = range_split.next().unwrap().parse().expect("sucks to suck");
        let max: i32 = range_split.next().unwrap().parse().expect("sucks to suck");

        let mut count = 0;
        let mut matches = 0;
        for (index, letter) in password.chars().enumerate() {
            if letter == target {
                count += 1;
                if ((index as i32) + 1) == min {
                    matches += 1;
                }
                if ((index as i32) + 1) == max {
                    matches += 1;
                }
            }
        }

        if count >= min && count <= max {
            valid_total += 1;
        }
        if matches == 1 {
            valid_total_part_2 += 1;
        }
    }
    println!("Day 2 Part 1: Number of valid passwords {}", valid_total);
    println!("Day 2 Part 2: Number of valid passwords {}", valid_total_part_2);
}

pub fn main() {
    let file_contents = file_contents_as_string("./data/day2.txt");
    parse_contents(&file_contents);
}
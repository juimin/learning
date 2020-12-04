use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

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

fn check_valid(current_pass_fields: &HashSet<&str>) -> bool {
    let required_fields: HashSet<&str> = [ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ].iter().cloned().collect();
    let intersection: HashSet<_> = required_fields.intersection(current_pass_fields).collect();
    // If the intersection contains all the values then we are good
    intersection.len() == required_fields.len()
}


fn validate_passports(fc: &str) -> i32 {
    let mut valid_passports = 0;
    let mut current_pass_fields: HashSet<&str> = HashSet::new();
    for line in fc.lines() {
        if line == "" {
            if check_valid(&current_pass_fields) {
                valid_passports += 1
            }
            // Finished this passport so reset the hashset
            current_pass_fields.clear();
        } else {
            // Parse the line to see if there are fields to add
            let elements = line.split_whitespace();
            for element in elements {
                let category = element.split(":").next().unwrap();
                current_pass_fields.insert(category.clone());
            }
        }
    }

    // Depending on the new line state of the end of the file we might have to check
    if current_pass_fields.len() > 0 {
        if check_valid(&current_pass_fields) {
            valid_passports += 1
        }
    }
    return valid_passports
}

pub fn main() {
    let fc = file_contents_as_string("./data/day4.txt");
    let part1 = validate_passports(&fc);
    println!("Day 4 Part 1: {}", part1);
}
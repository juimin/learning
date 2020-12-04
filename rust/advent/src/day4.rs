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

fn check_categories_valid(categories: &HashSet<&str>) -> bool {
    let req_fields: HashSet<&str> = ["byr","iyr","eyr","hgt","hcl","ecl","pid"].iter().cloned().collect();
    let intersection: HashSet<_> = req_fields.intersection(categories).collect();
    // If the intersection contains all the values then we are good
    intersection.len() == req_fields.len()
}

// Check each field is valid input
fn check_field_valid(cat: &str, val: &str) -> bool {
    match cat {
        "byr" => {
            if val.len() != 4 {
                return false
            }
            match val.trim().parse::<i32>() {
                Ok(n) => {
                    n >= 1920 && n <= 2002
                },
                Err(_) => false,
            }
        },
        "iyr" => {
            if val.len() != 4 {
                return false
            }
            match val.trim().parse::<i32>() {
                Ok(n) => {
                    n >= 2010 && n <= 2020
                },
                Err(_) => false,
            }
        },
        "eyr" => {
            if val.len() != 4 {
                return false
            }
            match val.trim().parse::<i32>() {
                Ok(n) => {
                    n >= 2020 && n <= 2030
                },
                Err(_) => false,
            }
        },
        "hgt" => {
            let len = val.len();
            match &val[len-2..] {
                "in" => {
                    match val[..len-2].trim().parse::<i32>() {
                        Ok(n) => {
                            n >= 59 && n <= 76
                        },
                        Err(_) => false,
                    }
                },
                "cm" => {
                    match val[..len-2].parse::<i32>() {
                        Ok(n) => {
                            n >= 150 && n <= 193
                        },
                        Err(_) => false,
                    }
                },
                _ => false
            }
        },
        "hcl" => {
            if val.len() != 7 {
                return false
            }
            let mut chars = val.chars();
            if chars.next().unwrap() != '#' {
                return false
            }
            // Iterate over the next
            let valid_hex: HashSet<char> = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'].iter().cloned().collect();
            for c in chars {
                if !valid_hex.contains(&c) {
                    return false
                }
            }
            true
        },
        "ecl" => {
            let eye_colors: HashSet<&str> = ["amb","blu","brn","gry","grn","hzl","oth"].iter().cloned().collect();
            eye_colors.contains(val)
        },
        "pid" => {
            // ensures length
            if val.len() != 9 {
                return false
            }
            // ensures is a number
            match val.trim().parse::<i32>() {
                Ok(_) => true,
                Err(_) => false,
            }
        },
        "cid" => true,
        _ => false,
    }
}


fn validate_passports(fc: &str) -> (i32, i32) {
    let mut valid_passports_1 = 0;
    let mut valid_passports_2 = 0;
    let mut part1_set: HashSet<&str> = HashSet::new();
    let mut part2_set: HashSet<&str> = HashSet::new();
    for line in fc.lines() {
        if line == "" {
            if check_categories_valid(&part1_set) {
                valid_passports_1 += 1
            }
            if check_categories_valid(&part2_set) {
                valid_passports_2 += 1
            }
            // Finished this passport so reset the sets
            part1_set.clear();
            part2_set.clear();
        } else {
            // Parse the line to see if there are fields to add
            let elements = line.split_whitespace();
            for element in elements {
                let mut item = element.split(":");
                let category = item.next().unwrap();
                let value = item.next().unwrap();
                if check_field_valid(category, value) {
                    part2_set.insert(category.clone());
                }
                part1_set.insert(category.clone());
            }
        }
    }

    // Depending on the new line at the end of the file we might have to check
    if part1_set.len() > 0 {
        if check_categories_valid(&part1_set) {
            valid_passports_1 += 1
        }
    }
    if part2_set.len() > 0 {
        if check_categories_valid(&part2_set) {
            valid_passports_2 += 1
        }
    }
    return (valid_passports_1, valid_passports_2)
}

pub fn main() {
    let fc = file_contents_as_string("./data/day4.txt");
    let (part1, part2) = validate_passports(&fc);
    println!("Day 4 Part 1: {}", part1);
    println!("Day 4 Part 2: {}", part2);
}
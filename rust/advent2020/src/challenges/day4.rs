use std::collections::HashSet;


fn check_categories_valid(categories: &HashSet<&str>) -> bool {
    let req_fields: HashSet<&str> = ["byr","iyr","eyr","hgt","hcl","ecl","pid"].iter().cloned().collect();
    let intersection: HashSet<_> = req_fields.intersection(categories).collect();
    // If the intersection contains all the values then we are good
    intersection.len() == req_fields.len()
}

fn const_len_val_in_range(val: &str, low_inc: i32, high_inc: i32, len: usize) -> bool {
    if val.len() != len {
        return false
    }
    val_in_range(val, low_inc, high_inc)
}

fn val_in_range(val: &str, low_inc: i32, high_inc: i32) -> bool {
    match val.trim().parse::<i32>() {
        Ok(n) => {
            n >= low_inc && n <= high_inc
        },
        Err(_) => false,
    }
}

// Check each field is valid input
fn check_field_valid(cat: &str, val: &str) -> bool {
    match cat {
        "byr" => const_len_val_in_range(val, 1920, 2002, 4),
        "iyr" => const_len_val_in_range(val, 2010, 2020, 4),
        "eyr" => const_len_val_in_range(val, 2020, 2030, 4),
        "hgt" => {
            let len = val.len();
            match &val[len-2..] {
                "in" => val_in_range(&val[..len-2], 59, 76),
                "cm" => val_in_range(&val[..len-2], 150, 193),
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


pub fn run(fc: &str) -> (i64, i64) {
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
    (valid_passports_1, valid_passports_2)
}
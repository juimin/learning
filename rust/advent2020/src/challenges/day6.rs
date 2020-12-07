use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Lines};

fn get_part2_group_count(arr: [i32; 26], size: i32) -> i64 {
    let mut total = 0;
    for cnt in arr.iter() {
        if *cnt == size {
            total += 1;
        }
    }
    total
}

pub fn run(lines: Lines<BufReader<File>>)  -> (i64, i64) {
    let mut results: (i64, i64) = (0,0);
    let mut part1_hs: HashSet<char> = HashSet::new();
    let mut part2_arr: [i32; 26] = [0; 26];
    let mut group_size: i32 = 0;

    for line in lines {
        if let Ok(l) = line {
            if l.len() == 0 {
                // This is a break so we can compute
                results.0 += part1_hs.len() as i64;
                part1_hs.clear();
    
                // Process part 2
                results.1 += get_part2_group_count(part2_arr, group_size);
                part2_arr.iter_mut().for_each(|m| *m = 0);
                group_size = 0;
            } else {
                // Add to set because this is part of the group
                for c in l.chars() {
                    part1_hs.insert(c);
                    // Use the part 2 version
                    part2_arr[(c as usize) - 97] += 1;
                }
                group_size += 1;
            }
        }
    }

    if part1_hs.len() != 0 {
        results.0 += part1_hs.len() as i64;
    }

    if part2_arr.len() != 0 {
        results.1 += get_part2_group_count(part2_arr, group_size);
    }
    results
}
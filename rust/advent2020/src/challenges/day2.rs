use std::fs::File;
use std::io::{BufReader, Lines};

pub fn run(lines: Lines<BufReader<File>>) -> (i64,i64) {
    // Get the valid password count
    let mut results: (i64, i64) = (0,0);
    for line in lines {
        if let Ok(l) = line {
            let mut splits = l.split_whitespace();
            // Extract the segments from the line
            let mut range_split = splits.next().unwrap().split("-");
            let target = splits.next().unwrap().chars().next().unwrap();
            let password = splits.next().unwrap();
    
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
                results.0 += 1;
            }
            if matches == 1 {
                results.1 += 1;
            }
        }
    }
    results
}
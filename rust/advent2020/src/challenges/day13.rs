use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashSet;
use std::vec::Vec;
use std::iter::FromIterator;

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results = (i64::max_value(),0);

    let mut part1_bus_id = 0;
    let mut target = -1;

    let mut ride_ids: Vec<i64> = Vec::new();

    // Read inputs
    for (idx, l) in lines.enumerate() {
        if let Ok(line) = l {
            if idx == 0 {
                target = line.parse().unwrap();
            }
            if idx == 1 {
                for t in line.split(",") {
                    if t != "x" {
                        let v: i64 = t.parse().unwrap();
                        let lower_limit = v * (target / v);
                        let upper_limit = v * ((target / v) + 1);
                        if lower_limit == target {
                            results.0 = 0;
                            break
                        }
                        if (upper_limit - target) < results.0 {
                            results.0 = upper_limit - target;
                            part1_bus_id = v;
                        }
                        ride_ids.push(v);
                    } else {
                        ride_ids.push(-1);
                    }
                }
            }
        }
    }

    let sorted_ids = Vec::from_iter(ride_ids.iter());

    let mut multiplier = 1;
    while results.1 == 0 {
        let mut failed = false;
        let start = sorted_ids[0] * multiplier;
        for idx in 1..sorted_ids.len() {
            if *sorted_ids[idx] != -1 {
                let v = start + idx as i64;
                if v % sorted_ids[idx] != 0 {
                    failed = true;
                    break;
                }
            }
        }
        if !failed {
            results.1 = start;
        }
        multiplier += 1;
    }


    results.0 *= part1_bus_id;

    results
}
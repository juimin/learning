use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::{HashSet, VecDeque};

fn two_sum(set: &HashSet<i64>, target: i64) -> bool {
    // Returns if the set contains 2 numbers summing to target
    for n in set {
        if set.contains(&(target - n)) {
            return true;
        }
    }
    false
}

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results = (0,0);

    // Process Part 1
    let mut window: HashSet<i64> = HashSet::new();
    let mut ordering: VecDeque<i64> = VecDeque::new();
    for line in lines {
        if let Ok(l) = line {
            let v: i64 = l.trim().parse().unwrap();
            if window.len() < 25 {
                ordering.push_back(v);
                window.insert(v);
            } else {
                // Once we have 25 values in we can do real processing
                if results.0 == 0 && !two_sum(&window, v) {
                    results.0 = v
                }
                // Move the window
                match ordering.pop_front() {
                    Some(v) => window.remove(&v),
                    None => true,
                };
                ordering.push_back(v);
                window.insert(v);
            }
        }
    }

    results
}
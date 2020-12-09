use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::{HashSet, VecDeque};
use std::vec::Vec;

fn two_sum(set: &HashSet<i64>, target: i64) -> bool {
    // Returns if the set contains 2 numbers summing to target
    for n in set {
        if set.contains(&(target - n)) {
            return true;
        }
    }
    false
}

fn target_sum_range(collection: &Vec<i64>, target: i64) -> (usize, usize) {
    let mut front = 0;
    let mut back = 0;
    let mut current_sum = 0;
    let mut results = (0,0);
    while front <= collection.len() && results == (0,0) {
        // Make sure we are always at least 2 away
        if front - back <= 2 {
            // Advance the front
            current_sum += collection[front];
            front += 1;
        } else {
            // Advance front qhile less
            if current_sum < target {
                current_sum += collection[front];
                front += 1;
            } else if current_sum > target {
                // Advance  the back if we are too big
                current_sum -= collection[back];
                back += 1;
            } else {
                // If we get here that means we are equal
                results = (back, front)
            }
        }

    }
    results
}

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results = (0,0);

    let mut num_collection: Vec<i64> = Vec::new();
    // Process Part 1
    let mut window: HashSet<i64> = HashSet::new();
    let mut ordering: VecDeque<i64> = VecDeque::new();
    for line in lines {
        if let Ok(l) = line {
            let v: i64 = l.trim().parse().unwrap();
            num_collection.push(v);
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

    let (a, b) = target_sum_range(&num_collection, results.0);
    // Find min and max
    match num_collection[a..b].iter().max() {
        Some(v) => results.1 += v,
        None => ()
    }
    match num_collection[a..b].iter().min() {
        Some(v) => results.1 += v,
        None => ()
    }

    results
}
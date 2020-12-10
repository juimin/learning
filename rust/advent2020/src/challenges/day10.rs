use std::fs::File;
use std::io::{BufReader, Lines};
use std::vec::Vec;

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results = (0,0);

    let mut adapters: Vec<i64> = Vec::new();

    for line in lines {
        if let Ok(l) = line {
            adapters.push(l.trim().parse().unwrap());
        }
    }
    
    // Sort the inputs
    adapters.sort();

    let mut diffs: [i64; 3] = [0; 3];
    let mut last = 0;
    for adapter in adapters {
        diffs[(adapter - last - 1) as usize] += 1;
        last = adapter;
    }
    // Remember that the device is +3 the last one
    diffs[2] += 1;

    results.0 = diffs[0] * diffs[2];

    results
}
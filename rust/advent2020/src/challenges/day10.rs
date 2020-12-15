use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::{HashMap};
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
    let phone_joltage = adapters[adapters.len() - 1] + 3;
    adapters.push(phone_joltage);

    let mut diffs: [i64; 3] = [0; 3];
    let mut last = 0;
    for adapter in &adapters {
        diffs[(adapter - last - 1) as usize] += 1;
        last = *adapter;
    }

    results.0 = diffs[0] * diffs[2];
    let mut mem: HashMap<i64, i64> = HashMap::new();
    let size = adapters.len() as i64;

    adapters.insert(0, 0);
    results.1 = solve_problem(&mut mem, &adapters, size, 0);

    results
}

fn solve_problem(memo: &mut HashMap<i64, i64>, adapters: &[i64], size: i64, index: i64) -> i64 {
    if index == size - 1 {
        return 1
    }
    // If we have already looked past here we can put in the results
    if memo.contains_key(&index) {
        let dict_entry = *memo.get(&index).unwrap();
        return dict_entry
    }
    // Check the next possible results
    let mut sum = 0;
    for offset in 1..4 {
        let next = offset + index;
        if next < size && adapters[next as usize] - adapters[index as usize] <= 3 {
            sum += solve_problem(memo, adapters, size, next);
        }
    }
    // Store the sum and return
    memo.insert(index, sum);

    // Return the sum
    sum
}
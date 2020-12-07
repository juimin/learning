use std::fs::File;
use std::io::{BufReader, Lines};

const TRIAL_COUNT: usize = 5;
const TRIALS: [(i32, i32); TRIAL_COUNT] = [(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)];

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut tree_counts: [i64; TRIAL_COUNT] = [0; TRIAL_COUNT];
    let mut trial_coordinates: [(i32, i32); TRIAL_COUNT] = [(0, 0); TRIAL_COUNT];
    for line in lines {
        if let Ok(l) = line {
            for (trial, (long_step, lat_step)) in TRIALS.iter().enumerate() {
                if trial_coordinates[trial].0 == *long_step {
                    // Move laterally
                    trial_coordinates[trial].1 =
                        (trial_coordinates[trial].1 + lat_step) % (l.len() as i32);
                    // Check the new spot for a tree
                    if l.chars().nth(trial_coordinates[trial].1 as usize).unwrap() == '#' {
                        tree_counts[trial] += 1
                    }
                    // Reset long_pos
                    trial_coordinates[trial].0 = 0;
                }
                trial_coordinates[trial].0 += 1;
            }
        }
    }
    let mut part2 = 1;
    for n in tree_counts.iter() {
        part2 *= n;
    }
    (tree_counts[0], part2)
}

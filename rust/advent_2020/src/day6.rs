use crate::util;

use std::collections::HashSet;

pub fn run() {
    let fc = util::file_contents_as_string("./data/day6.txt");

    let mut part1_sum = 0;
    let mut hs: HashSet<char> = HashSet::new();

    for line in fc.lines() {
        if line.len() == 0 {
            // This is a break so we can compute
            part1_sum += hs.len();
            hs.clear();
        } else {
            // Add to set because this is part of the group
            for c in line.chars() {
                hs.insert(c);
            }
        }
    }

    if hs.len() != 0 {
        part1_sum += hs.len();
    }

    println!("Day 6 Part 1: {}", part1_sum);
}
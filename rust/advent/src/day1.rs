use std::collections::HashSet;
use std::vec::Vec;

use crate::util;

pub fn main() {
    let file_contents = util::file_contents_as_string("./data/day1.txt");
    let target_sum = 2020;
    let mut seen: HashSet<i32> = HashSet::new();
    for line in file_contents.split_whitespace() {
        // Check if diff is in the set
        let n = line.trim().parse().expect("This should be a number");
        let other = target_sum - n;
        if seen.contains(&other) {
            println!("Day 1 Part 1: {}", n * other);
        }
        seen.insert(n);
    }

    let mut new_targets = Vec::new();
    // We can do this because the number are unique
    for n in &seen {
        let int_n = (*n).clone();
        let diff_n = target_sum - int_n;
        new_targets.push((int_n, diff_n));
    }

    // This is ugly as shit
    for (n, diff_target) in new_targets {
        for x in &seen {
            let n2 = (*x).clone();
            if n2 != n {
                let n3 = diff_target - n2;
                if seen.contains(&n3) {
                    if n3 != n2 && n != n3 {
                        // We have found the triple
                        println!("Day 1 Part 2: {}", n * n2 * n3);
                        return
                    }
                }
            }
        }
    }
}
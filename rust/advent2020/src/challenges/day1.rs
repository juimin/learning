use std::collections::HashSet;
use std::vec::Vec;

pub fn run(file: &str) -> (i64,i64) {
    let target_sum = 2020;
    let mut seen: HashSet<i32> = HashSet::new();
    let mut results: (i64, i64) = (0, 0);
    for line in file.lines() {
        // Check if diff is in the set
        let n = line.trim().parse().expect("This should be a number");
        let other = target_sum - n;
        if seen.contains(&other) {
            results.0 = (n * other) as i64;
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
                        results.1 = (n * n2 * n3) as i64;
                        return results;
                    }
                }
            }
        }
    }
    results
}
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let path = Path::new("./data/day1.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            // Solve the problem
            let target_sum = 2020;

            let mut seen: HashSet<i32> = HashSet::new();

            for line in file_contents.split_whitespace() {
                // Check if diff is in the set
                let n = line.trim().parse().expect("This should be a number");
                let other = target_sum - n;
                if seen.contains(&other) {
                    println!("Day 1 Target Product: {}", n * other);
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
                                println!("Day 1 Triple Product: {}", n * n2 * n3);
                            }
                        }
                    }
                }
            }
        },
    }
    // `file` goes out of scope, and the "hello.txt" file gets closed
}
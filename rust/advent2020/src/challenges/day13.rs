use std::fs::File;
use std::io::{BufReader, Lines};

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results = (i64::max_value(),0);

    let mut part1_bus_id = 0;
    let mut target = -1;
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
                    }
                }
            }
        }
    }

    results.0 *= part1_bus_id;
    println!("{}", target);

    results
}
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let results = (0,0);
    for line in lines {
        let Ok(l) = line {
            println!("DO STUFF");
        }
    }
    results
}
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn file_contents_as_string(filename: &str) -> String {
    let path = Path::new(filename);
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
        Ok(_) => return file_contents
    }
}

fn count_trees(file_contents: &str) {
    const LAT_STEP: i32 = 3;
    const LONG_STEP: i32 = 1;
    let mut tree_count = 0;
    let mut lat_pos = 0;
    let mut long_pos = 0;
    for line in file_contents.lines() {
        if long_pos == LONG_STEP {
            // Move laterally
            lat_pos = (lat_pos + LAT_STEP) % (line.len() as i32);
            // Check the new spot for a tree
            if line.chars().nth(lat_pos as usize).unwrap() == '#' {
                tree_count += 1
            }
            // Reset long_pos
            long_pos = 0;
        }
        long_pos += 1;
    }

    println!("Day 3 Part 1: Tree count is {}", tree_count);
}

pub fn main() {
    let file_contents = file_contents_as_string("./data/day3.txt");
    count_trees(&file_contents);
}
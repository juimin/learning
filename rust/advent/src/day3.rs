use crate::util;

fn count_trees(file_contents: &str, lat_step: i32, long_step: i32) -> i32 {
    let mut tree_count = 0;
    let mut lat_pos = 0;
    let mut long_pos = 0;
    for line in file_contents.lines() {
        if long_pos == long_step {
            // Move laterally
            lat_pos = (lat_pos + lat_step) % (line.len() as i32);
            // Check the new spot for a tree
            if line.chars().nth(lat_pos as usize).unwrap() == '#' {
                tree_count += 1
            }
            // Reset long_pos
            long_pos = 0;
        }
        long_pos += 1;
    }

    return tree_count;
}

pub fn main() {
    let file_contents = util::file_contents_as_string("./data/day3.txt");
    let part1 = count_trees(&file_contents, 3, 1);
    println!("Day 3 Part 1: Tree count is {}", part1);
    // Part 2
    let trials = [(1,1), (3, 1), (5, 1), (7, 1), (1,2)];
    let mut part2_product: i64 = 1;
    for t in trials.iter() {
        let cnt = count_trees(&file_contents, t.0, t.1);
        part2_product = part2_product * (cnt as i64);
    }
    println!("Day 3 Part 2: Tree count is {}", part2_product);
}
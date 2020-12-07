fn count_trees(file_contents: &str, lat_step: i32, long_step: i32) -> i64 {
    let mut tree_count: i64 = 0;
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
    tree_count
}

pub fn run(file: &str) -> (i64, i64) {
    let part1 = count_trees(file, 3, 1);
    // Part 2
    let trials = [(1,1), (3, 1), (5, 1), (7, 1), (1,2)];
    let mut part2_product: i64 = 1;
    for t in trials.iter() {
        let cnt = count_trees(file, t.0, t.1);
        part2_product = part2_product * cnt;
    }
    (part1, part2_product)
}
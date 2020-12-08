use std::fs::File;
use std::io::{BufReader, Lines};

fn split_root_and_contents(line: &str) -> (&str, &str) {
    let mut root_and_contents = line.split("contain");
    let this_bag = root_and_contents.next().unwrap().trim();
    let contents = root_and_contents.next().unwrap().trim();
    (this_bag, contents)
}

fn clean_bag_color(bag_str: &str) -> &str {
    bag_str.trim().split("bag").next().unwrap().trim()
}

fn get_count_and_clean(bag_str: &str) -> (i32, &str) {
    let count_str = bag_str.trim().split_whitespace().next().unwrap();
    (
        count_str.parse().unwrap(),
        clean_bag_color(bag_str.split(count_str).nth(1).unwrap()),
    )
}

// Assumes valid input
pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let result = (0, 0);
    for line in lines {
        if let Ok(line) = line {
            let (mut this_bag, contents) = split_root_and_contents(&line);
            this_bag = clean_bag_color(this_bag);
            for c in contents.split(",") {
                if !c.contains("no other bags") {
                    let (_, child_bag) = get_count_and_clean(c);
                }
            }
        }
    }

    result
}

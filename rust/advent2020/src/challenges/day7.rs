use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::vec::Vec;

const TARGET_BAG: &str = "shiny gold";

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
    let mut result = (0, 0);

    let mut part1_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        if let Ok(line) = line {
            let (mut this_bag, contents) = split_root_and_contents(&line);
            this_bag = clean_bag_color(this_bag);
            for c in contents.split(",") {
                if !c.contains("no other bags") {
                    let (_, child_bag) = get_count_and_clean(c);
                    // Tackle part 1
                    // Here we map children to their parents
                    let parents = part1_map.entry(String::from(child_bag)).or_insert(HashSet::new());
                    parents.insert(String::from(this_bag));
                }
            }
        }
    }

    // Part 1 Processing
    let mut seen: HashSet<String> = HashSet::new();
    let mut todo: Vec<String> = Vec::new();
    todo.push(String::from(TARGET_BAG));
    while let Some(top) = todo.pop() {
        match part1_map.get(&top) {
            Some(parents) => {
                for parent in parents {
                    todo.push(String::from(parent));
                }
            }
            None => ()
        }
        seen.insert(String::from(&top));
    }

    // Subtract one because we don't count the original bag
    result.0 = seen.len() as i64 - 1;
    result
}

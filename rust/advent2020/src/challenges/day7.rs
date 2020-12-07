use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::vec::Vec;

fn get_bag_color(bag_str: &str) -> &str {
    bag_str.split("bag").next().unwrap().trim()
}

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results = (0, 0);
    let mut bag_to_parent: HashMap<String, HashSet<String>> = HashMap::new();
    for line in lines {
        if let Ok(l) = line {
            let mut root_bag = l.split("contain");
            let root_bag_color = get_bag_color(root_bag.next().unwrap());
            let bag_rules = root_bag.next().unwrap().trim();
            if !bag_to_parent.contains_key(root_bag_color) {
                bag_to_parent.insert(String::from(root_bag_color), HashSet::new());
            }
            if bag_rules == "no other bags." {
                println!("ignore {}", root_bag_color);
            } else {
                // Parse bag rules
                for rule in bag_rules.split(",") {
                    let mut bag_characteristics = rule.split_whitespace();
                    let count = bag_characteristics.next().unwrap();
                    let color = get_bag_color(rule.split(count).nth(1).unwrap().trim());
                    if !bag_to_parent.contains_key(color) {
                        bag_to_parent.insert(String::from(color), HashSet::new());
                    }
                    if let Some(parents) = bag_to_parent.get_mut(color) {
                        parents.insert(String::from(root_bag_color));
                    }
                }
            }
        }
    }
    let mut seen: HashSet<String> = HashSet::new();
    let mut stack: Vec<&str> = Vec::new();
    stack.push(&"shiny gold");
    while let Some(top) = stack.pop() {
        if !seen.contains(top) {
            for parent in bag_to_parent[top].iter() {
                if !seen.contains(parent) {
                    results.0 += 1;
                    stack.push(parent);
                }
            }
            seen.insert(String::from(top));
        }
    }

    results
}

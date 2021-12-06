use std::collections::HashMap;

use adventlib;

fn get_all_points(moves: &Vec<String>) -> HashMap<(i64, i64), i64> {
    let mut results = HashMap::new();
    let mut current_position = (0,0);

    let mut steps = 0;
    for m in moves {
        let dist = m.trim()[1..m.len()].parse::<i64>().unwrap();
        let mut mask = (0,0);
        let direction = m.chars().nth(0).unwrap();
        match direction {
            'R' => {
                mask.1 += 1;
            },
            'L' => {
                mask.1 -= 1;
            },
            'D' => {
                mask.0 -= 1;
            },
            'U' => {
                mask.0 += 1;
            },
            _ => {
                println!("we're toast")
            }
        }
        for _ in 0..dist {
            steps += 1;
            current_position.0 += mask.0;
            current_position.1 += mask.1;
            if !results.contains_key(&current_position) {
                results.insert(current_position.clone(),steps);
            }
        }
    }

    return results;
}

pub fn crossed_wires(file: &String) {
    println!("Crossed Wires\n");
    let contents = adventlib::read_comma_sep_lines(file);
    
    let first_wire_points = get_all_points(&contents[0]);
    let second_wire_points = get_all_points(&contents[1]);

    let mut min_dist = -1;
    for point in first_wire_points.keys() {
        if second_wire_points.contains_key(point) {
            let a = (0 - point.0).abs();
            let b = (0 - point.1).abs();
            let manhattan_distance = a + b;
            if min_dist < 0 || manhattan_distance < min_dist {
                min_dist = manhattan_distance;
            }
        }
    }

    println!("Minimum manhattan distance: {}", min_dist);

    let mut min_steps = -1;
    for point in first_wire_points.keys() {
        if second_wire_points.contains_key(point) {
            let total_steps = first_wire_points[point] + second_wire_points[point];
            if min_steps < 0 || total_steps < min_steps {
                min_steps = total_steps;
            }
        }
    }

    println!("Minimum step distance: {}", min_steps);
}
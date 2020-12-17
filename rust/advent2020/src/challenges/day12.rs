use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results = (0,0);

    let mut heading = Direction::East;
    let mut position = (0,0);

    for line in lines {
        if let Ok(l) = line {
            let command = l.chars().nth(0).unwrap();
            let units: i64 = l[1..].parse().unwrap();
            match command {
                'N' => {position.1 += units},
                'S' => {position.1 -= units},
                'E' => {position.0 += units},
                'W' => {position.0 -= units},
                'L' => {
                    for _ in 0..(units / 90) {
                        match heading {
                            Direction::North => {heading = Direction::West},
                            Direction::East => {heading = Direction::North},
                            Direction::South => {heading = Direction::East},
                            Direction::West => {heading = Direction::South},
                        }
                    }
                },
                'R' => {
                    for _ in 0..(units / 90) {
                        match heading {
                            Direction::North => {heading = Direction::East},
                            Direction::East => {heading = Direction::South},
                            Direction::South => {heading = Direction::West},
                            Direction::West => {heading = Direction::North},
                        }
                    }
                }
                _ => (),
            }
            if command == 'F' {
                match heading {
                    Direction::North => {position.1 += units},
                    Direction::East => {position.0 += units},
                    Direction::South => {position.1 -= units},
                    Direction::West => {position.0 -= units},
                }
            }
        }
    }

    results.0 = position.0.abs() + position.1.abs();

    results
}
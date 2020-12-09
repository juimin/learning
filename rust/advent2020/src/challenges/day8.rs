use std::fs::File;
use std::io::{BufReader, Lines};
use std::vec::Vec;
use std::collections::HashSet;

#[derive(Debug)]
enum Operation {
    Nop,
    Jump,
    Acc,
}

#[derive(Debug)]
struct Instruction {
    value: i64,
    op: Operation,
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let mut split = s.split_whitespace();
        let op = split.next().unwrap();
        let val = split.next().unwrap();
        Instruction {
            op: match op {
                "nop" => Operation::Nop,
                "acc" => Operation::Acc,
                "jmp" => Operation::Jump,
                _ => Operation::Nop
            },
            value: val.trim().parse().unwrap()
        }
    }
}

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results = (0,0);

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        if let Ok(l) = line {
            instructions.push(Instruction::from_str(&l));
        }
    }

    let mut swappable: Vec<i64> = Vec::new();
    for (index, ins) in instructions.iter().enumerate() {
        match ins.op {
            Operation::Nop => swappable.push(index as i64),
            Operation::Jump => swappable.push(index as i64),
            _ => (),
        }
    }

    let instruction_size = instructions.len() as i64;

    // Part 1 Solution
    let mut idx: i64 = 0;
    let mut seen: HashSet<i64> = HashSet::new();
    while idx < instruction_size && !seen.contains(&idx) {
        let instruction = &instructions[idx as usize];
        seen.insert(idx);
        match instruction.op {
            Operation::Nop => idx += 1,
            Operation::Jump => idx += instruction.value,
            Operation::Acc => {
                results.0 += instruction.value;
                idx += 1
            },
        }
    }

    // Part 2 Solution
    for swap_idx in swappable {
        let mut idx: i64 = 0;
        let mut acc = 0;
        let mut seen: HashSet<i64> = HashSet::new();
        while idx < instruction_size && !seen.contains(&idx) {
            let instruction = &instructions[idx as usize];
            let mut op = &instruction.op;
            if idx == swap_idx {
                match op {
                    Operation::Jump => op = &Operation::Nop,
                    Operation::Nop => op = &Operation::Jump,
                    _ => (),
                }
            }
            seen.insert(idx);
            match op {
                Operation::Nop => idx += 1,
                Operation::Jump => idx += instruction.value,
                Operation::Acc => {
                    acc += instruction.value;
                    idx += 1
                },
            }
        }
        if idx >= instruction_size {
            results.1 = acc;
        }
    }


    results
}
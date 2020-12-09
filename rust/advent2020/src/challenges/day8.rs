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
    Value: i64,
    Op: Operation,
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let mut split = s.split_whitespace();
        let op = split.next().unwrap();
        let val = split.next().unwrap();
        println!("{} {}", op, val);
        Instruction {
            Op: match op {
                "nop" => Operation::Nop,
                "acc" => Operation::Acc,
                "jmp" => Operation::Jump,
                _ => Operation::Nop
            },
            Value: val.trim().parse().unwrap()
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

    let mut idx: i64 = 0;
    let mut seen: HashSet<i64> = HashSet::new();
    let instruction_size = instructions.len() as i64;
    while idx < instruction_size && !seen.contains(&idx) {

        let instruction = &instructions[idx as usize];
        seen.insert(idx);
        match instruction.Op {
            Operation::Nop => idx += 1,
            Operation::Jump => idx += instruction.Value,
            Operation::Acc => {
                results.0 += instruction.Value;
                idx += 1
            },
            _ => panic!("What the fuck"),
        }
    }

    results
}
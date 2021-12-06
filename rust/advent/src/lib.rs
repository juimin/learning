use std::fs;

// Base Reader
pub fn read_file_with_sep(file: &String, sep: &str) -> Vec<String> {
    // Read the contents from the given file
    let contents = fs::read_to_string(file).expect("Error reading file");
    let contents = contents.trim();
    // Split the values, convert to string, filter empty strings, and collect them
    return contents.split(sep).map(|s| s.to_string()).collect();
}

// Derived Types
pub fn read_file(file: &String) -> Vec<String> {
    return read_file_with_sep(file, "\n")
}

pub fn read_file_as_list(file: &String) -> Vec<String> {
    return read_file_with_sep(file, ",")
}

// More specific
pub fn read_file_as_i64(file: &String) -> Vec<i64> {
    let values = read_file(file);
    return values.iter().map(|s| s.parse::<i64>().unwrap()).collect()
}

pub fn read_file_as_list_of_i64(file: &String) -> Vec<i64> {
    let values = read_file_as_list(file);
    return values.iter().map(|s| s.parse::<i64>().unwrap()).collect()
}

pub fn read_comma_sep_lines(file: &String) -> Vec<Vec<String>> {
    let values = read_file(file);
    let mut output = Vec::new();
    for line in &values {
        output.push(Vec::new());
        let idx = output.len() - 1;
        let line_values: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
        for s in  &line_values {
            output[idx].push(s.clone())
        }
    }

    return output;
}

// From 2019 Problem 2 and 5
fn run_intcode(intcode: &mut Vec<i64>) {
    let mut op_code_index = 0;
    while op_code_index < intcode.len() {
        match intcode[op_code_index] {
            1 => {
                let a = intcode[op_code_index + 1];
                let b = intcode[op_code_index + 2];
                let location = intcode[op_code_index + 3];
                intcode[location as usize] = intcode[a as usize] + intcode[b as usize];
            },
            2 => {
                let a = intcode[op_code_index + 1];
                let b = intcode[op_code_index + 2];
                let location = intcode[op_code_index + 3];
                intcode[location as usize] = intcode[a as usize] * intcode[b as usize];
            },
            _ => return
        }

        // step to the next opcode
        op_code_index += 4
    }
}

// From 2019 Problem 2
pub fn run_calculation(intcode: &mut Vec<i64>, p1: i64, p2: i64) {
    // Seed values because instructions say so. comment out for testing
    intcode[1] = p1;
    intcode[2] = p2;
    run_intcode(intcode);
}
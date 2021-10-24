use adventlib;

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

fn run_calculation(intcode: &mut Vec<i64>, p1: i64, p2: i64) {
    // Seed values because instructions say so. comment out for testing
    intcode[1] = p1;
    intcode[2] = p2;
    run_intcode(intcode);
}

fn find_target(seedcode: &Vec<i64>, target: i64, noun_limit: i64, verb_limit: i64) -> i64 {
    // Arbitrary limits
    for noun in 0..noun_limit {
        for verb in 0..verb_limit {
            let mut intcode = seedcode.clone();
            run_calculation(&mut intcode, noun, verb);
            if intcode[0] == target {
                println!("Matched target {} with verb ({}) and noun ({})", target, verb, noun);
                return 100 * noun + verb;
            }
        }
    }
    return 0;
}


pub fn program_alarm(file: &String) {
    println!("1202 Program Alarm\n");

    let seedcode = adventlib::read_file_as_list_of_i64(file);

    let mut part_one_intcode = seedcode.clone();
    run_calculation(&mut part_one_intcode, 12, 2);

    println!("Position 0 value at termination: {}", part_one_intcode[0]);

    let target = find_target(&seedcode, 19690720, 100, 100);

    println!("The final answer of (100 * noun + verb): {}", target);

}
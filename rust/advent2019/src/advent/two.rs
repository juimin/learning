use adventlib;

fn run_intcode(intcode: &mut Vec<i64>) {
    let mut op_code_index = 0;
    while op_code_index < intcode.len() {
        let command = intcode[op_code_index];
        let a = intcode[op_code_index + 1];
        let b = intcode[op_code_index + 2];
        let location = intcode[op_code_index + 3];
        match command {
            1 => {
                intcode[location as usize] = a + b;
            },
            2 => {
                intcode[location as usize] = a * b;
            },
            _ => return
        }

        // step to the next opcode
        op_code_index += 4
    }
}


pub fn program_alarm(file: &String) {
    println!("1202 Program Alarm\n");

    let mut intcode = adventlib::read_file_as_list_of_i64(file);

    run_intcode(&mut intcode);


    println!("Position 0 value at termination: {}", intcode[0]);

}
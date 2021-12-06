use adventlib;



fn find_target(seedcode: &Vec<i64>, target: i64, noun_limit: i64, verb_limit: i64) -> i64 {
    // Arbitrary limits
    for noun in 0..noun_limit {
        for verb in 0..verb_limit {
            let mut intcode = seedcode.clone();
            adventlib::run_calculation(&mut intcode, noun, verb);
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
    adventlib::run_calculation(&mut part_one_intcode, 12, 2);

    println!("Position 0 value at termination: {}", part_one_intcode[0]);

    let target = find_target(&seedcode, 19690720, 100, 100);

    println!("The final answer of (100 * noun + verb): {}", target);

}
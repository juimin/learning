use adventlib;

pub fn sonar_sweep(file: &String) {
    let inputs = adventlib::file_to_int_vector(file, "\n");

    let mut increases = 0;
    let mut last_input = i64::MAX;

    for input in &inputs {
        if *input > last_input {
            increases += 1;
        }
        last_input = *input
    }

    println!("Number of inputs: {}", increases)
}

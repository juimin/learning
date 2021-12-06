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

    println!("Number of increases: {}", increases);

    increases = 0;
    let mut last_sum = i64::MAX;

    for i in 0..inputs.len() - 2 {
        let sum = &inputs[i] + &inputs[i + 1] + &inputs[i + 2];

        if sum > last_sum {
            increases += 1;
        }
        last_sum = sum;
    }

    println!("Number of increases for windows: {}", increases);
}

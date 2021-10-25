use adventlib;

fn solve_passwords(arg_one: i64, arg_two: i64) -> i64 {
    let mut total = 0;

    for i in arg_one..arg_two {
        let s = i.to_string();
        // Guaranteeds to be lower than a numeral in ascii
        let mut last_char = '+';

        let len_check = s.len() == 6;
        let mut adjacent_repeat_check = false;
        let mut ascending = true;

        for c in s.chars() {
            if c == last_char {
                adjacent_repeat_check = true;
            }
            if (c as i64) < (last_char as i64) {
                ascending = false;
            }

            last_char = c;
        }      
        
        if len_check && adjacent_repeat_check && ascending {
            total += 1;
        }
    }

    return total;
}

fn solve_part_two(arg_one: i64, arg_two: i64) -> i64 {
    let mut total = 0;

    for i in arg_one..arg_two {
        let s = i.to_string();
        // Guaranteeds to be lower than a numeral in ascii
        let mut last_char = '+';

        let len_check = s.len() == 6;
        let mut ascending = true;

        let mut exactly_two = false;
        let mut adjacent_count = 0;

        for c in s.chars() {
            if c == last_char {
                adjacent_count += 1;
            } else {
                if adjacent_count == 2 {
                    exactly_two = true;
                }
                adjacent_count = 1;
            }

            if (c as i64) < (last_char as i64) {
                ascending = false;
            }

            last_char = c;
        }      

        if adjacent_count == 2 {
            exactly_two = true;
        }
        
        if len_check && exactly_two && ascending {
            total += 1;
        }
    }

    return total;
}


pub fn secure_container(file: &String) {
    println!("Secure Container\n");
    let inputs = adventlib::read_file(file);

    for input in inputs {
        let digits: Vec<String> = input.split("-").map(|s| s.to_string()).collect();
        let arg_one = digits[0].parse::<i64>().unwrap();
        let arg_two = digits[1].parse::<i64>().unwrap();
        println!("Passwords in range {} - {}: {}", arg_one, arg_two, solve_passwords(arg_one, arg_two));
        println!("Passwords in range {} - {}: {}", arg_one, arg_two, solve_part_two(arg_one, arg_two));
           
    }
}
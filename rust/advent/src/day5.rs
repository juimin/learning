use crate::util;

// Parses the 7 letter code
fn get_row_number(input: &str) -> i32 {
    binary_search(128, 'B', input)
}


fn get_col_number(input: &str) -> i32 {
    binary_search(8, 'R', input)
}


fn binary_search(size: i32, high: char, input: &str) -> i32 {
    let mut min = 0;
    let mut max = size;
    let mut mid = (max + min) / 2;

    for letter in input.chars() {
        if letter == high {
            min = mid;
            mid = (max + min) / 2;
        } else {
            max = mid;
            mid = (max + min) / 2;
        }
    }
    return mid
}


pub fn run() {
    let fc = util::file_contents_as_string("./data/day5.txt");
    let mut max_seat_id = 0;
    // Compute colum xor sum
    let mut xor_sum = 0;
    for n in 1..9 {
        xor_sum ^= n;
    }

    let mut counter: [i32; 128] = [xor_sum; 128];

    for line in fc.lines() {
        let len = line.len();
        let row = get_row_number(&line[..7]);
        let col = get_col_number(&line[7..len]);
        let seat_id = (row * 8) + col;
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
        // Add to the counter
        counter[(row as usize)] ^= col + 1;
    }
    println!("Day 5 Part 1: {}", max_seat_id);

    let mut in_middle = false;
    for (row, value) in counter.iter().enumerate() {
        if !in_middle && *value == 0 {
            in_middle = true;
        }
        // The first one we find should be the correct one
        if in_middle && *value != 0 {
            let my_seat_id = (8 * (row as i32)) + value - 1;
            println!("Day 5 Part 2: {}", my_seat_id);
            return
        }
    }
}
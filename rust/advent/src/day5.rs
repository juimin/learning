use crate::util;


fn bin_conv(high: char, input: &str) -> i32 {
    let mut val = 0;

    // bleh signed and unsigned ints
    for letter in input.chars() {
        if letter == high {
            val |= 1
        }
        val <<= 1;
    }
    val >>= 1;
    return val
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
        let row = bin_conv('B', &line[..7]);
        let col = bin_conv('R', &line[7..len]);
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
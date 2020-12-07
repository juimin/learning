use std::fs::File;
use std::io::{BufReader, Lines};

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
    return val;
}

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results: (i64, i64) = (0, 0);
    let mut max_seat_id = 0;
    // Compute colum xor sum
    let mut xor_sum = 0;
    for n in 1..9 {
        xor_sum ^= n;
    }

    let mut counter: [i32; 128] = [xor_sum; 128];

    for line in lines {
        if let Ok(l) = line {
            let len = l.len();
            let row = bin_conv('B', &l[..7]);
            let col = bin_conv('R', &l[7..len]);
            let seat_id = (row * 8) + col;
            if seat_id > max_seat_id {
                max_seat_id = seat_id;
            }
            // Add to the counter
            counter[(row as usize)] ^= col + 1;
        }
    }

    results.0 = max_seat_id as i64;

    let mut in_middle = false;
    for (row, value) in counter.iter().enumerate() {
        if in_middle {
            if *value != 0 {
                results.1 = ((8 * (row as i32)) + value - 1) as i64;
                return results;
            }
        } else {
            if *value < xor_sum {
                in_middle = true;
            }
        }
    }
    results
}

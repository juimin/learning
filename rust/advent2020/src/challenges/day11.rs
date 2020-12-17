use std::fs::File;
use std::io::{BufReader, Lines};
use std::vec::Vec;


fn get_adjacent_indexes(list: &mut Vec<(usize,usize)>, current: (usize,usize), limits: (usize,usize)) {
    let (r, c) = current;
    let (row_lim, col_lim) = limits;
    for r_offset in -1..2 {
        for c_offset in -1..2 {
            if !(r_offset == 0 && c_offset == 0) {
                let new_r = r as i64 + r_offset;
                let new_c = c as i64 + c_offset;
                if (new_r >= 0 && new_r < row_lim as i64) && (new_c >= 0 && new_c < col_lim as i64) {
                    list.push((new_r as usize, new_c as usize));
                }
            }
        }
    }
}

fn new_seat_state(matrix: &Vec<Vec<char>>, limits: (usize,usize), current: (usize, usize)) -> char {
    let result = matrix[current.0][current.1];
    if result == '.' {
        return result
    }
    let mut adjacents = Vec::new();
    get_adjacent_indexes(&mut adjacents, current, limits);
    let mut count_occupied = 0;
    for (x, y) in adjacents {
        if matrix[x as usize][y as usize] == '#' {
            count_occupied += 1
        }
    }
    if result == 'L' && count_occupied == 0 {
        return '#'
    }
    if result == '#' && count_occupied >= 4 {
        return 'L'
    }
    result
}

fn new_seat_state2(matrix: &Vec<Vec<char>>, limits: (usize,usize), current: (usize, usize)) -> char {
    let result = matrix[current.0][current.1];
    if result == '.' {
        return result
    }
    let mut adjacents = Vec::new();

    // Obtain adjacent by line of sight

    let (r, c) = current;
    let (row_lim, col_lim) = limits;
    for r_offset in -1..2 {
        for c_offset in -1..2 {
            let mut found = false;
            // Here we should have a valid heading
            if !(r_offset == 0 && c_offset == 0) {
                let mut mult = 1;
                let mut new_r = r as i64 + r_offset;
                let mut new_c = c as i64 + c_offset;
                while !found && (new_r >= 0 && new_r < row_lim as i64) && (new_c >= 0 && new_c < col_lim as i64) {
                    if matrix[new_r as usize][new_c as usize] != '.' {
                        adjacents.push((new_r as usize, new_c as usize));
                        found = true;
                    }
                    mult += 1;
                    new_r = r as i64 + (r_offset* mult);
                    new_c = c as i64 + (c_offset * mult);
                }
            }
        }
    }


    let mut count_occupied = 0;
    for (x, y) in adjacents {
        if matrix[x as usize][y as usize] == '#' {
            count_occupied += 1
        }
    }
    if result == 'L' && count_occupied == 0 {
        return '#'
    }
    if result == '#' && count_occupied >= 5 {
        return 'L'
    }
    result
}

fn solve_part_1(matrix: &mut Vec<Vec<char>>) -> i64 {
    // Process part 1
    let mut result = 0;
    let row_len = matrix.len();
    let col_len = matrix[0].len();
    let mut changes: i64 = -1;

    while changes != 0 {
        changes = 0;
        let mut copy_matrix: Vec<Vec<char>> = Vec::new();
        for row_index in 0..row_len {
            copy_matrix.push(Vec::new());
            for col_index in 0..col_len {
                let current = matrix[row_index][col_index];
                let seat_update = new_seat_state(&matrix, (row_len, col_len), (row_index, col_index));
                if seat_update != current {
                    changes += 1;
                }
                copy_matrix[row_index].push(seat_update);
            }
        }
        for row in 0..copy_matrix.len() {
            for col in 0..copy_matrix[row].len() {
                matrix[row][col] = copy_matrix[row][col]
            }
        }
    }


    for row in matrix {
        for seat in row {
            if *seat == '#' {
                result += 1
            }
        }
    }
    result
}

fn solve_part_2(matrix: &mut Vec<Vec<char>>) -> i64 {
    // Process part 1
    let mut result = 0;
    let row_len = matrix.len();
    let col_len = matrix[0].len();
    let mut changes: i64 = -1;
    while changes != 0 {
        changes = 0;
        let mut copy_matrix: Vec<Vec<char>> = Vec::new();
        for row_index in 0..row_len {
            copy_matrix.push(Vec::new());
            for col_index in 0..col_len {
                let current = matrix[row_index][col_index];
                let seat_update = new_seat_state2(&matrix, (row_len, col_len), (row_index, col_index));
                if seat_update != current {
                    changes += 1;
                }
                copy_matrix[row_index].push(seat_update);
            }
        }
        for row in 0..copy_matrix.len() {
            for col in 0..copy_matrix[row].len() {
                matrix[row][col] = copy_matrix[row][col]
            }
        }
    }


    for row in matrix {
        for seat in row {
            if *seat == '#' {
                result += 1
            }
        }
    }
    result
}

pub fn run(lines: Lines<BufReader<File>>) -> (i64, i64) {
    let mut results = (0,0);

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut matrix2: Vec<Vec<char>> = Vec::new();

    for (i, line) in lines.enumerate() {
        if let Ok(l) = line {
            // Build the matrix
            matrix.push(Vec::new());
            matrix2.push(Vec::new());
            // We should assume the index is correct as we build 1 by 1
            for character in l.chars() {
                matrix[i].push(character);
                matrix2[i].push(character);
            }
        }
    }

    results.0 = solve_part_1(&mut matrix);
    results.1 = solve_part_2(&mut matrix2);

    results
}
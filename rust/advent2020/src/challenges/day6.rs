use std::collections::HashSet;

fn get_part2_group_count(arr: [i32; 26], size: i32) -> i32 {
    let mut total = 0;
    for cnt in arr.iter() {
        if *cnt == size {
            total += 1;
        }
    }
    total
}

pub fn run(file: &str) -> (i64, i64) {
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut hs: HashSet<char> = HashSet::new();
    let mut part2_arr: [i32; 26] = [0; 26];
    let mut group_size: i32 = 0;

    for line in file.lines() {
        if line.len() == 0 {
            // This is a break so we can compute
            part1_sum += hs.len();
            hs.clear();

            // Process part 2
            part2_sum += get_part2_group_count(part2_arr, group_size);
            part2_arr.iter_mut().for_each(|m| *m = 0);
            group_size = 0;
        } else {
            // Add to set because this is part of the group
            for c in line.chars() {
                hs.insert(c);
                // Use the part 2 version
                part2_arr[(c as usize) - 97] += 1;
            }
            group_size += 1;
        }
    }

    if hs.len() != 0 {
        part1_sum += hs.len();
    }

    if part2_arr.len() != 0 {
        part2_sum += get_part2_group_count(part2_arr, group_size);
    }
    (part1_sum as i64, part2_sum as i64)
}
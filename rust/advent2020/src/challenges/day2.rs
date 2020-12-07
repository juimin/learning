pub fn run(file_contents: &str) -> (i64,i64) {
    // Get the valid password count
    let mut valid_total = 0;
    let mut valid_total_part_2 = 0;
    for line in file_contents.lines() {
        let mut splits = line.split_whitespace();
        // Extract the segments from the line
        let p1 = splits.next();
        let mut range_split = p1.unwrap().split("-");
        let p2 = splits.next();
        let target = p2.unwrap().chars().next().unwrap();
        let p3 = splits.next();
        let password = p3.unwrap();

        // Get the range
        let min: i32 = range_split.next().unwrap().parse().expect("sucks to suck");
        let max: i32 = range_split.next().unwrap().parse().expect("sucks to suck");

        let mut count = 0;
        let mut matches = 0;
        for (index, letter) in password.chars().enumerate() {
            if letter == target {
                count += 1;
                if ((index as i32) + 1) == min {
                    matches += 1;
                }
                if ((index as i32) + 1) == max {
                    matches += 1;
                }
            }
        }

        if count >= min && count <= max {
            valid_total += 1;
        }
        if matches == 1 {
            valid_total_part_2 += 1;
        }
    }
    (valid_total, valid_total_part_2)
}
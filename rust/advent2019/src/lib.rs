use std::fs;

pub fn read_file(file: &String) -> Vec<String> {
    // Read the contents from the given file
    let contents = fs::read_to_string(file).expect("Error reading file");
    let contents = contents.trim();
    // Split the values, convert to string, filter empty strings, and collect them
    return contents.split("\n").map(|s| s.to_string()).collect();
}

pub fn read_file_as_i64(file: &String) -> Vec<i64> {
    let values = read_file(file);
    return values.iter().map(|s| s.parse::<i64>().unwrap()).collect()
}
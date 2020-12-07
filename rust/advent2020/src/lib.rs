use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn file_contents_as_string(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => return file_contents
    }
}


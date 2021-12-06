use adventlib;

pub fn sunny_with_a_chance_of_asteroids(file: &String) {
    println!("Sunny with a chance of Asteroids\n");
    let diagnostic = adventlib::read_file_as_list(file);
    println!("{:?}", diagnostic);
}

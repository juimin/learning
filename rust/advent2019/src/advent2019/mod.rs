pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;

pub fn run(day: u8, file: &String) {
    match day {
        1 => one::rocket_equation(file),
        2 => two::program_alarm(file),
        3 => three::crossed_wires(file),
        4 => four::secure_container(file),
        5 => five::sunny_with_a_chance_of_asteroids(file),
        6 => println!("Not implemented\n"),
        7 => println!("Not implemented\n"),
        8 => println!("Not implemented\n"),
        9 => println!("Not implemented\n"),
        10 => println!("Not implemented\n"),
        11 => println!("Not implemented\n"),
        12 => println!("Not implemented\n"),
        13 => println!("Not implemented\n"),
        14 => println!("Not implemented\n"),
        15 => println!("Not implemented\n"),
        16 => println!("Not implemented\n"),
        17 => println!("Not implemented\n"),
        18 => println!("Not implemented\n"),
        19 => println!("Not implemented\n"),
        20 => println!("Not implemented\n"),
        21 => println!("Not implemented\n"),
        22 => println!("Not implemented\n"),
        23 => println!("Not implemented\n"),
        24 => println!("Not implemented\n"),
        25 => println!("Not implemented\n"),
        _ => println!("Not implemented\n")
    }
}
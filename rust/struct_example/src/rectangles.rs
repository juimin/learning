pub fn initial_version() {
    let w = 30;
    let h = 50;
    println!("The area of the rectangle is {}", area(w, h));
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}
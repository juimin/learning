/*
Functions

THe bread and butter of everthing

Statements vs expressions

Statements have no return while expressions do

Expressions don't need semicolons


*/

fn this_returns_something() -> u32 {
    let y = {
        let ozone = 3;
        ozone * 23
    };

    return {
        y + 222
    }
}


pub fn main() {
    println!("Functions here we come");
    let asf = this_returns_something();
    println!("We got something: {}", asf);
    println!();
}
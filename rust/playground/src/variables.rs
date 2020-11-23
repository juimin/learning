/*
Variables and Mutability

Variables
    - Immutable By Default


*/

fn immutable_example() {
    println!("immutability example:");
    // This shouldn't work because x is immutable
    // The variable x cannot change the value it points to
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // This will work
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!();
}

pub fn main() {
    println!("==Variables Module==\n");
    immutable_example();
}
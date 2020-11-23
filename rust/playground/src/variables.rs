/*
Variables and Mutability

Variables
    - Immutable By Default
    - The value that the variable points to cannot change

Constants
    - By definition immutable
    - Type annotation required
    - Can be declared in any scope
    - Can only be set to a constant expression
        - This is different from python where it can be set at run time

Shadowing
    - Declare a new variable with the same name as a previous variable
*/

// Constants are all caps (standard)
// underscores can be used in numeric literals for readability (this is still 100k)
const BASIC_GLOBAL: u32 = 100_000;


// This is similar to what we see in python
fn shadow_example() {
    println!("shadowing example:");
    // Note that the variable doesn't need to be mutable
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // Because we are doing a reassignment and creating a new variable
    // We can change to the type\
    let spaces = "   ";
    println!("The value of spaces is: {}", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
    println!();

    // Compare this to the following which will NOT work
    // let mut spaces = "   ";
    // println!("The value of spaces is: {}", spaces);
    // let spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);

    // This fails because the variable is mutable and we expect to be able to reassign
    // it but we cannot reassign it to a value of a different type

    // It seems that when you make a variable mutable, it becomes type locked
}


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

fn constants_example() {
    const LOCAL_CONST: u8 = 12;
    println!("constants example:");
    println!("basic global example: {}", BASIC_GLOBAL);
    println!("basic local example: {}", LOCAL_CONST);
    println!();
}


pub fn main() {
    println!("==Variables Module==\n");
    immutable_example();
    constants_example();
    shadow_example();
}
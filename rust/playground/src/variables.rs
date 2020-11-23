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
*/

// Constants are all caps (standard)
// underscores can be used in numeric literals for readability (this is still 100k)
const BASIC_GLOBAL: u32 = 100_000;


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
}


pub fn main() {
    println!("==Variables Module==\n");
    immutable_example();
    constants_example();
}
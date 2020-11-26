/*
References and Borrowing

How we pass variables around without derping everything

References
- Referencing a value creates a pointer to the existing object metadata on the stack
- So value is on the heap, the stack gets the ptr to the value, len, cap etc.
- & on the variable returns a pointer that points to the stack pointer

Dereferences
- * dereferences the pointers which resolves to the value itself
- You don't need to dereference all the time because rust knows what pointers lead to
    and can infer their methods

Mutable References
- You can only have one mutable reference to a value in a particular scope
    let r1 = &mut s;
    let r2 = &mut s;
    This is not allowed

    // The following circumvents this issue by wrapping one mutable reference in its own scope
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
*/


fn calculate_length(s: &String) -> usize {
    s.len()
}

fn pointer_example() {
    // This allows us to use s1 here and keep using it after passing it to calculate_length
    // Technically s1 is still owned by main here
    let s1 = String::from("hello");

    calculate_length2(&s1);
}

fn calculate_length2(s: &String) {
    println!("Dereferencing {}.", *s);
}

fn pointer_example2() {
    // This allows us to use s1 here and keep using it after passing it to calculate_length
    // Technically s1 is still owned by main here
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}


// Consider the following
fn changing() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    // Thid doesn't work because we can't modify immutable references
    //some_string.push_str(", world");
    println!("{}", some_string);
}

// The version below works because s is now mutable and we passed a mutable reference
// So the function has to know that it is modifying the original value

fn changing_mutable() {
    let mut s = String::from("hello");

    change_mutable(&mut s);
}

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world");
}

fn bad_double_borrow() {
    // This function doesn't work because you are trying to borrow one value
    // as both immutable and mutable
    //let mut s = String::from("hello");

    //let r1 = &s; // no problem
    //let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
}

fn good_double_borrow() {
    // This works because we use the immutable references before making a mutable reference
    // so r1 and r2 won't have unexpected changes to their borrowed objects
    // and r3 can do what it needs to

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    // The scopes of the immutable references r1 and r2 end after the println! where they are last used,

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn dangling_pointers() -> String {
    // References don't dangle in rust
    // Compiler ensures that the data goes out of scope before the reference does

    // This is good
    let s = String::from("hello");

    s
    // By comparison this won't work
    // let s = String::from("hello"); // s is a new String

    // &s // we return a reference to the String, s
    // This is because once we exit dangling_pointers scope, the value of s will no longer be valid and the pointer
    // will lead to nothing
    // In cases like this we should return the value, which passes ownership to the scope of the calling code
}

pub fn main() {
    println!("\nReferences and Borrowing");
    pointer_example();
    pointer_example2();
    changing();
    changing_mutable();
    bad_double_borrow();
    good_double_borrow();
    dangling_pointers();
}
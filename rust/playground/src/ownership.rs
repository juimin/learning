/*
Ownership the big daddy strat for memory control

Enables memory safe guarantees without a garbage collector
*/

/*
Ownership rules are checked at compile time and not runtime

Review the stack and the heap
Stack - structured and easy to allocate memory for
Heap - random as fuck
    - obviously finding a space to put stuff in the heap is expensive


Accessing data in the heap is also slower
    - Processors are faster if they jump around less (less ops)
    - Heap objects may be disjoint (think linked lists) so you might have to look around
        a lot for things you need
            - by comparison, everything in the stack is next to it's related data

managing heap data is why ownership exists
*/

/*
THE RULES

Each value in Rust has a variable that’s called its owner.

- Does this refer to the variable you instantiated?

There can only be one owner at a time.

- Hm... references must all be copies?

When the owner goes out of scope, the value will be dropped.

- Reference counting?
- No because there's only one reference so we don't need to count


variable assignment is copy reference by value
    - references are always stored in the stack
    - the heap data that the pointer points to is unchanged and uncopied
*/

fn invalidating_variables() {
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    // The above doesn't work because we invalidate the first copy
    // So we copy the pointer+metadata on the stack to s2, invalidate s1
    // and then we only have one copy of the information to work with

    // Assumption is that Rust never does deep copies automatically
}

fn deepcopy() {
    // There might be more functions that accomplish this but clone
    // could be considered the deep copy equivalent

    // In this case, a new pointer,len, and cap are created on the stack
    // and point to a completely new area of memory on the heap where we copied the data

    // Because it's dealing with the heap, deep copies are generally expensive
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_only_copy() {
    // Why does this work?
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // This works because all the memory is stored entirely on the stack
    // We don't have to worry about allocating/freeing memory on the heap
    // Once these values are no longer in scope they both go away

    // This is accomplished via the Copy trait, which exists on types like integer
    // that are stored only on the stack

    // Drop and Copy are two traits that cannot coexist on the same type

    // Primitives and scalar values are probably Copy traited
    // Anything more complicated that might require heap allocation is not Copy traited
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn ownershipchanging() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

    // println!("attempt to use s: {}", s);
    // Compiler error that the value has moved away
    println!("{}", x);
    // This is still fine though
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
        // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
        // moves out to the calling
        // function
}

fn moving_ownership() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    println!("{}", s3);
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.


// This is kind of annoying to have to do all the time in order to re-obtain
// ownership within this scope after passing a value to another function

// REFERENCES OBVIATE THIS
fn giveback() {
    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

pub fn main() {
    invalidating_variables();
    deepcopy();
    stack_only_copy();
    ownershipchanging();
    moving_ownership();
    giveback();
}
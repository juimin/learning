// Vectors Strings and Hash Maps

/*
Remember all of these own their own entries so you shouldn't store pointers to anything


Vectors
- Growable data structurs
- You shouldn't index because they are growable and might move on the heap if they need to be resized
- Basis for String types
- Can simulate Queues or Stacks
- You can have more than one type in a Vector by wrapping them in an enum type

String
- Used closely with String slices
- pointers to String are translated into pointers to string slices in compiler
- The backing is a vector of u8 ints
- You can't index them because utf8 characters are multibyte entities
    - converting with chars() gets you unicode scalars but not necessarily graphemes that would make sense
        - I think you only need to care about this if you are dealing with arabic or something similar

Hash Map
- Secure hash that's kinda slow but very secure

*/

pub fn main() {
    println!("Stuff about data structures");
}
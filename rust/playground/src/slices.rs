/*
Slices

Apparently slices don't have ownership

Slices let you reference contiguous sequences of elements in a collection over the whole collection

Slices are really just pointers to different points in a collection
    - They contain the pointer and a len for how long they are
*/

// Naive version of finding the first word
// Converts the string to bytes to iterate over and check through the items in the string
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// The correct way to write string parsing functions is to have them take &str
// &str is a reference to a string slice
fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn better_user() {
    // Attempt via string object
    let s = String::from("This is a string object");
    println!("s first word is {}", better_first_word(&s[..]));
    // Try with a string literal
    // String literals are themselves slices
    let sl = "Potato potahto";
    // Just using the straight up string literal works
    println!("s first word is {}", better_first_word(sl));

    // References to string literal's full slice
    println!("s first word is {}", better_first_word(&sl[..]));

}

pub fn main() {
    let s = String::from("something something potato");
    let idx = first_word(&s);
    println!("{}", idx);
    better_user();
}
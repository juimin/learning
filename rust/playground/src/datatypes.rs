// Data type introduction
/*
Scalars and Compounds

Rust is statically typed (thank god)
    - Sometimes it can guess the types based on the values
        - ex. string literals or integer literals don't need annotations
        - resolving an object from a function is more vague and requires
            annotations so we don't get compiler errors


Scalar Types
- Singular value
- Four types of scalars
    - Integers, Floats, Booleans, Characters

In release mode, integer overflow will result in automatic wrapping to prevent
panic errors (which we would see in debug mode)

There is a wrapping crate that facilitates this so don't use the release functionality
as part of the logic because this is considered a bug


Compound types
This is also pretty much what you expect
- Tuples and arrays

Tuple
    - FIxed length and is the general default for grouping several values of varying types
Array:
    - Fixed length and all values contained must be of the same type
    - Automatically go to the stack because all values are same type (size known) and the
    array itself has a defined size (so we can easily define it on the stack)

Vector (extra credit)
    - availabe in the standard library and is similar to the array except is resizable
*/


fn tuple_land() {
    println!("Tuple land");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("This is a tuple: {}", tup);
    // This doesn't work because the tuple doesn't implement display
    
    // Lets try destructuring the tuple
    let (a, b, c) = tup;
    println!("This is a tuple: {}, {}, {}", a, b, c);

    // We can also access via indexing but with the . not tup[0]

    println!("This is a tuple with indexing: {}, {}, {}", tup.0, tup.1, tup.2);

    println!();
}

fn arrayland() {
    // Also pretty much like what you already know
    // Grouping of like typed values
    // REQUIRE FIXED LENGTH
    println!("Array land");

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    // Arrays don't implement display either
    // I think at this point it should be safe to assume that all collections have this problem
    // println!("Months of the year: {}", months);
    println!("January == {}", months[0]);
    // Note that indexing is with brackets and not the dots.

    // let a: [i32; 5] = [1,2,3,4,5];
    // Type annotation for the array (or I guess part of the definition?)
    // The type and the size is described

    // Fast definition initialization
    // Creates an array with all the same value and some size
    let b = [5;10];
    // here b is an array of length 10 with all entries being 5
    for idx in 0..10 {
        println!("{}", b[idx]);
    }

    // Be wary of index out of bounds because this is probably possible
    // The following will panic at compile time
    // let a = [1, 2, 3, 4, 5];
    // let index = 10;

    // let element = a[index];

    // println!("The value of element is: {}", element);

    println!();
}


fn integers() {
    println!("Integers");
    /*
    We can have signed and unsigned integers of varying sizes
    Up to 128 bit integers for both
    obviously implemented using two's complement

    arch - what the heck is this?
        - it literally means what architectures is the system
        - most computers are either 32 or 64 bit
        - isize and usize correspond to signed and unsigned integers based on the
            architecture bit base
                - So my computer is running 64 bit everything so each would correspond
                - to i64 and u64
            - This is probably helpful for older systems but 64 bit is the modern age so
            - Ignore using anything less probably unless you need the software super portable
            
            - MOST COMMON USE CASE FOR isize and usize
                - indexing some kind of collection

    General observation is that 32 bit integers are still the best even on 64 bit systems
    */
    let x: isize = 23;
    println!("some integer using a system signed integer = {}", x);

    // Type suffix also works
    let x = 255u8;
    println!("unsigned 8 bit integer max = {}", x);

    // Lets try to break it

    // let x: u8 = 0;
    // for a in 0..256 {
    //     let x = x + a;
    // }

    // Interestingly, the compiler knows that this will break
    println!();
}

fn floats() {
    println!("Floats all around");
    // Two types, f32 and f64 with f64 being the default
    // IEEE-754 standard for floating point numbers
    // 32 bit is single precision while 64 is double
    // XXX: What the heck does this even mean
    println!();
}

fn operations() {
    println!("Operations");
    // addition
    let x = 5 + 10;
    println!("addition = {}", x);
    // subtraction
    let x = 95.5 - 4.3;
    println!("subtraction = {}", x);
    // multiplication
    let x = 4 * 30;
    println!("multiplication = {}", x);
    // division
    let x = 56.7 / 32.2;
    println!("division = {}", x);
    let x = 9/10; // integer division still in play here
    println!("integer division 9/10 = {}", x);
    let x = 2.0/3.0; // float division works as expected
    println!("float division = {}", x);
    // remainder
    let x = 43 % 5;
    println!("remainder = {}", x);
    let x = 9 % 10;
    println!("remainder = {}", x);
    println!();
}

fn booleans() {
    println!("Boolean fun");
    let t = true;
    let f: bool = false; // There isn't a reason to type annotate this really
    // These work pretty much how you would expect
    println!("true = {}", t);
    println!("remainder = {}", f);
    println!("are t and f the same? = {}", t == f);
    println!();
}

fn characterjazz() {
    println!("Char stuff");
    // This is probably most like the rune seen in go. It's technically a primative
    // Like c, these should be specified as single quote variables
    // 4 byte unicode scalar value
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
    println!();
}


pub fn main() {
    println!("Scalars");
    integers();
    floats();
    operations();
    booleans();
    characterjazz();
    tuple_land();
    arrayland();
}
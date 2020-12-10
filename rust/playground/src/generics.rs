
// Func definition for the largest whatever T is in a sequence
fn largest<T>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        // Remember that this isn't going to work unless we can guarantee that T has a PartialOrd Trait
        // basically they have to implement some functional comparison
        if item > max {
            max = item
        }
    }
    max
}

// See how we can define the types here to be whatever we want
// Note that x and y need to have the same type. If they differ in this generic situation the code won't compile
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// Sample enum def

enum Option<T> {
    Some(T),
    None,
}

// Multi type
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Here we m ake points with integers and floats
fn pointmain() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
}

pub fn main() {

    pointmain();


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
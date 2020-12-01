/*
Chapter 6 is about enumerators

Allows definition of types by specification of variants

Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.

Options
    - A kind of enum (built in?)
    - Expresses that a value can be something or nothing (optional type from python)

match expression
    - switch statement

if let
    - syntactic sugar?
*/

pub fn main() {
    println!("\nEnumerations and all that Jazz\n");
    instantiate_example();
    better_ipaddr_enum();
    mixed_enum();
    use_message();
    try_out_options();
    run_value_in_cents();
    run_state_value_stuff();
    use_plus_one();
}

// Sample enum
#[derive(PartialEq)]
enum IpAddrKind {
    V4,
    V6,
}

fn printip(ip_kind: IpAddrKind) -> String {
    if ip_kind == IpAddrKind::V4 {
        return String::from("V4");
    } else {
        return String::from("V6");
    }
    
}

fn instantiate_example() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Six: {} - Four: {}", printip(six), printip(four));
}

// Associate each of the enum kinds with a value type
// This eliminates the need for extra structs to specify an ip address

// Otherwise it would be something like
/*
struct Ipaddr {
    IpaddrKind: IpAddrKind::V4,
    Value: String,
}

*/
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn better_ipaddr_enum() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    // Can we print these?
    println!("These are the better ipaddr enums: {:?} {:?}", home, loopback);
}

#[derive(Debug)]
enum MixedIpAddr {
    // See how we can make the type a tuple
    V4(u8, u8, u8, u8),
    V6(String),
}


fn mixed_enum() {
    let home = MixedIpAddr::V4(127, 0, 0, 1);

    let loopback = MixedIpAddr::V6(String::from("::1"));

    // Can we print these?
    println!("These are the better mixed ipaddr enums: {:?} {:?}", home, loopback);
}

// STANDARD LIBRARY IP ADDR DEFINITION ( in case you were curious )
/*
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
*/

#[derive(PartialEq)]
enum Message {
    // This is just a kind of message
    Quit,
    // This is an anonymous struct
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// The above is roughly equivalent to making a bunch of independent structs
/*
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

impl Message {
    fn call(&self) {
        // method body would be defined here
        if *self == Message::Quit {
            println!("Quitting");
        } else {
            println!("Not a quit message");
        }
    }
}

fn use_message() {
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Quit;
    m.call();
    let m = Message::ChangeColor(10,10,10);
    m.call();
    let m = Message::Move {x: 2, y: 213};
    m.call();
}

// The option enum!!!!!!!! We're finally here
/*
Nulls lmao

Rust does not have nulls, but instead has Option, which is an enum used to encode the concept
of a value being present or absent

As defined in the standard library (it can take any type)
enum Option<T> {
    Some(T),
    None,
}

Option is included in the prelude so you don't need to bring it into scope explicitly
Some and None can be used without the Option:: prefix
    - This is useful because Python is also using None and it won't be as hard to change
    - Note the generic <T>
*/

fn try_out_options() {

    // This is valid because the type used is an integer (default u32)
    let some_number = Some(5);
    let some_string = Some("a string");

    // This defines a variable with a specified type associated with the None value
    // We know with the annotation that the number will be None or a signed 32 bit integer
    let absent_number: Option<i32> = None;

    println!("some_number {:?} some_string {:?}", some_number, some_string);
    println!("absent_number {:?}", absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // We can't do this because the option doesn't allow addition
    // let sum = x + y;
    if y != None {
        let res = y.unwrap_or(0);
        println!("the sum is {}", x + res);
    }

    // In general, rust makes you be very deliberate about handling variables.
}

// MATCH CONTROL FLOW
// Essentially the switch statement except for enums

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // Recognize that these are evaluated in order
    match coin {
        // Brackets can be used if you need more logic
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn run_value_in_cents() {
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter;

    println!("Value of coin is {}", value_in_cents(p));
    println!("Value of coin is {}", value_in_cents(n));
    println!("Value of coin is {}", value_in_cents(q));
    println!("Value of coin is {}", value_in_cents(d));
}

// Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. 
// This is how we can extract values out of enum variants.

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum StateCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn state_value_in_cents(coin: StateCoin) -> u8 {
    match coin {
        StateCoin::Penny => 1,
        StateCoin::Nickel => 5,
        StateCoin::Dime => 10,
        StateCoin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn run_state_value_stuff() {
    let p = StateCoin::Penny;
    let n = StateCoin::Nickel;
    let d = StateCoin::Dime;
    let q = StateCoin::Quarter(UsState::Alaska);
    let q2 = StateCoin::Quarter(UsState::Alabama);

    println!("Value of coin is {}", state_value_in_cents(p));
    println!("Value of coin is {}", state_value_in_cents(n));
    println!("Value of coin is {}", state_value_in_cents(q));
    println!("Value of coin is {}", state_value_in_cents(d));
    println!("Value of coin is {}", state_value_in_cents(q2));
}

// Now use the match statement to modify something inside an Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    // Here we get x, which is an optional
    // If none, we return none obviously

    // If we get a Some of type T, in this case i32, we use that value as i
    // and return another optional with that value
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn use_plus_one() {
    // We define some optional holding an i32 (default)
    let five = Some(5);
    // Let six be the +1 variant
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is equal to {:?}", six);
    println!("none is equal to {:?}", none);
}


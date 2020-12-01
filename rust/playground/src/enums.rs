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
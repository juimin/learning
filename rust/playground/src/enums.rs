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

pub fn main() {
    println!("\nEnumerations and all that Jazz\n");
    instantiate_example();
}
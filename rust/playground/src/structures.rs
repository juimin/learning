/*
Structs (or objects)

All fields in a struct must be mutable

- Note that structs should have ownership over all it's elements
which is why using a reference to something in a struct is generally not allowed by the compiler

- There is a way to get the compiler to allow structs to store references by using lifetimes
which are something discussed in a later chapter
*/

struct ExampleStruct {
    field: u32,
    string_field: String,
    string_literal: String,
    some_boolean: bool,

}

fn use_example() {
    // Make a mutable example struct
    // Once a struct is mutable, everything in it is mutable
    let mut ex = ExampleStruct {
        field: 22,
        string_field: String::from("Banana"),
        string_literal: String::from("something something something"),
        some_boolean: true,
    };

    println!("examples: string_literal: {} {} {} {}", ex.string_literal, ex.field, ex.string_field, ex.some_boolean);

    // Modify the struct
    // This doesn't work if ex is not mutable
    ex.string_field.push('w');
    println!("examples: string_literal: {} {} {} {}", ex.string_literal, ex.field, ex.string_field, ex.some_boolean);
}

// This example uses the shorthand that we sometimes see in other languages
// If the field name and variable name are the same, we don't need to use keywords
// This also shows that we can use string literals in this &str fashion
fn shorthand_example(field: u32, string_field: String, sl: &str, some_boolean: bool) {
    let string_literal = String::from(sl);
    let ex = ExampleStruct {
        field, string_field, string_literal, some_boolean
    };

    println!("examples: string_literal: {} {} {} {}", ex.string_literal, ex.field, ex.string_field, ex.some_boolean);

}

fn update_syntax() {
    let mut ex = ExampleStruct {
        field: 22,
        string_field: String::from("Banana"),
        string_literal: String::from("something something something"),
        some_boolean: true,
    };

    // We can do this
    // This copies all the data in ex to ex2
    let ex2 = ExampleStruct {
        string_field: String::from("abasdf"),
        string_literal: String::from("asswswwq"),
        ..ex
    };

    // Here, we have copied the copyable fields to ex2
    // Note that we can't copy String because String doesn't implement the Copy Trait
    println!("examples: string_literal: {} {} {} {}", ex.string_literal, ex.field, ex.string_field, ex.some_boolean);
    ex.string_field.push('z');
    println!("examples: string_literal: {} {} {} {}", ex.string_literal, ex.field, ex.string_field, ex.some_boolean);

    // All the same information that we copied, field and some_boolean, are the same
    println!("examples: string_literal: {} {} {} {}", ex2.string_literal, ex2.field, ex2.string_field, ex2.some_boolean);
}


// TUPLE STRUCTS ( I guess like namedtuples)

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct_ex() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

pub fn main() {
    use_example();
    shorthand_example(22, String::from("potato"), "somethingh", false);
    update_syntax();
    tuple_struct_ex();
}
// Initial Version

pub fn initial_version() {
    println!("Initial Version");
    let w = 30;
    let h = 50;
    println!("The area of the ghetto rectangle is {}", area(w, h));
}

// This is bad from a design stand point because rectangles should be able
// to tell their own areas and we don't need a util function to calculate it
// when we could attach it to rectangles inherently
fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

// Tuple Version

// This version is a little better because we are using one arg instead of two
// but perhaps it still isn't the cleanest
fn tuple_area(rect: (u32, u32)) -> u32 {
    return rect.0 * rect.1;
}

pub fn tuple_version() {
    println!("Tuple Version");
    let rect = (30, 50);
    println!("The area of the tuple rectangle is {}", tuple_area(rect));
    let r = TupleRectangle(30, 50);
    println!("The area of the tuple struct rectangle is {}", tuple_struct_area(&r));
    
}

struct TupleRectangle(u32, u32);

fn tuple_struct_area(rect: &TupleRectangle) -> u32 {
    rect.0 * rect.1
}


// Struct Version
struct Rectangle {
    width: u32,
    height: u32,
}


// This version includes the derived debug annotation
#[derive(Debug)]
struct DebugRectangle {
    width: u32,
    height: u32,
}

pub fn struct_version() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Struct Version");
    println!("The area of the struct rectangle is {}", struct_area(&rect));
}

fn struct_area(rect: &Rectangle) -> u32 {
    // Remember the final statement is a return and we don't have to specify return
    rect.width * rect.height
}


// Version with traits
pub fn trait_version() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Struct with Traits Version");
    // Regular {} will attempt to use the Display Trait
    // {:?} will attempt to use the Debug trait (why debug for pretty print?)
        // This seems to be the json ish version
    // {:#?} is a further option we could use to display with more detail (struct name) 
    let debug_rect = DebugRectangle {
        width: 30,
        height: 50,    
    };
    println!("Test print the struct {:#?}", debug_rect);
    println!("The area of the struct rectangle is {}", struct_area(&rect));
}
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


// Implement the area function on the Rectangle itself

/* REMEMBER THIS
However, methods are different from functions in that they’re defined within the context of a struct 
(or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), 
and their first parameter is always self, which represents the instance of the struct the method is being called on.
*/
impl DebugRectangle {
    // See how this rectangle implements the area method
    // Remember methods always take the first arg as the reference to the object instance
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Version with traits
pub fn trait_version() {
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
    println!("The area of the rectangle using a struct method is {}", debug_rect.area());
}

// This version includes the derived debug annotation
#[derive(Debug)]
struct DubiousDebugRectangle {
    width: u32,
    height: u32,
    area: u32,
}

impl DubiousDebugRectangle {
    fn calc_area(&mut self) -> u32 {
        self.area = self.height * self.width;
        self.area
    }
}

pub fn dubious_version() {
    println!("Struct with Dubious Traits Version");
    // Regular {} will attempt to use the Display Trait
    // {:?} will attempt to use the Debug trait (why debug for pretty print?)
        // This seems to be the json ish version
    // {:#?} is a further option we could use to display with more detail (struct name) 
    let mut debug_rect = DubiousDebugRectangle {
        width: 30,
        height: 50,
        area: 0,
    };
    println!("The area of the dubious rectangle using a struct method before {}", debug_rect.area); 
    debug_rect.calc_area();
    println!("Test print the struct {:#?}", debug_rect);
    println!("The area of the dubious rectangle using a struct method after {}", debug_rect.area); 
}
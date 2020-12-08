/*

Error handling

There are two kinds of errors, recoverable and unrecoverable
    - unrecoverable is almost always a bug
        - panic!
    - recoverable is like when you are unable to open a file and can retry etc.
        - Results<T, E>


panic!
    This is actually a macro
    macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

    Unwinding
    - This cleans up the memory usage by the program rather than having the OS do it
    
    If you want the OS to do it and keep the program really small you can add this to the .toml file
        [profile.release]
        panic = 'abort'

    You can follow the stack trace. Hopefully you found everything before you create the release version
    '
Result<T, E>
    T = Type of value if success happens
    E = Type of error

    fn main() {
        let f = File::open("hello.txt");
    }

    opening a file returns a result

    f here will be std::result::Result<std::fs::File, std::io::Error> based on the std lib
        - Here we see that the sucessful return is a file handle and if error occurs we get the std lib io::Error

    // This is one way that we can catch file problems
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

*/
use std::fs::File;
use std::io::{self, ErrorKind, Error};
use std::io::Read;

fn tryfile() {
//    let f = File::open("hello.txt");

    //let f = match f {
    //    Ok(file) => file,

        // Err(error) => panic!("Problem opening the file: {:?}", error),
    //};
    println!("try file notes");
}

/*

    Example Branching on error handling

    use std::io::ErrorKind;

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    This is a better version after you learn what closures are

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    expect and unwrap are kinda the same except you can spec your own error message

    Errors can be returned (obviously)

    fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

*/

// ? Notation requires errors to have a from function

fn read_username_from_file() -> Result<String, io::Error> {
    // This is kinda confusing
    // If it doesn't give you an error, it returns the error?
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn main() {
    println!("Handle those errors");
    tryfile();
}
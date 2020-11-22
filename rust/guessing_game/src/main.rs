use std::io;
use rand::Rng; // Refer to chapter 10 for why we do this import this way
use std::cmp::Ordering;

fn main() {
    
    // Generate the random number

    // thread_rng is a random number generator in the local thread
    // (I think we are still single threaded at this point)
    // RNG is seeded by the operating system
    let secret_number = rand::thread_rng().gen_range(1,101);
    // left bound is inclusive, right bound is exclusive
    // println!("The secret number: {}", secret_number);

    // This is for printing out the prompt
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");

        // Obtaining input from the console
        // let is used to make variables
        // mut means the variable is mutable
            // default is immutable variables
        let mut guess = String::new();
        // String::new() returns a string type (built in)
        // This is a 'growable' type and defaults utf8

        // :: is an associated function
            // essentially a class function and not an instance function
            // STATIC METHOD

        // TESTING STRING LITERAL
        // let static_string = "godzilla";
        // println!("TEST: printing {}", static_string);

        // This is how you read in std input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        // println can handle format strings
        println!("You guessed: {}", guess);

        // second guess using module->type referencing rather than implied from import
        // let mut guess2 = String::new();
        // std::io::stdin()
        //     .read_line(&mut guess2)
        //     .expect("Failed to read line again.");
        // This section returns io::Result
            // Subclass of the generic Result type
            // Results are enums
            // Ok and err variants of Results


        // println!("You guessed: {}", guess2);

        // Further notes

        // & indicates a reference (pointer) as you are used to
        // &mut guess makes the pointer mutable (why is this necessary now?)

        // Convert the guess to an integer (unsigned 32 bit int)
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // Variable shadowing is allowed
        // : u32 is a type annotation and communicates what parse() should coerce the string into
        // expect again here handles the Result out of parse, printing the error string and exiting
        // trim() and parse() are instance functions is an instance function

        // Add comparison between types
        // Compare the guess variable with a reference to secret_number
        // The ref doesn't need to be mutable because we aren't writing anything
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

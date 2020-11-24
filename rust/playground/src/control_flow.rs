/*
Control Flow baby

if statements everywhere

There are also match expressions (I guess these are switch cases?)

Conditions must be run on bools (so truthy and falsey is going to be kinda derp)
*/

fn control_flow_ex() {
    let number = 6;

    // Short circuiting is a thing
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn ternary() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // This is broken because 5 and "six" are of different types
    // Ternary expressions must always use the same type

    // let number = if condition { 5 } else { "six" };
}


fn loopyland() {
    // loop
    // This is infinite until you break out

    // mutable counter
    let mut counter = 0;

    // You can return an expression from the loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // I guess this is like a return?
        }
    };

    println!("The result is {}", result);
}

fn whileloops() {
    // Obviates the logic of using breaks and the infinite loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Example number 2 using indexing
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}


fn for_loops() {
    // Define some collection to iterate over
    let a = [10, 20, 30, 40, 50];

    // This returns an iterator over the iterable
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Second example using a range and a reversing function
    // Note that left is inclusive right is exclusive
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn fibbonaci(n: u32) -> u32 {
    if n == 0 {
        return 1
    } else if n == 1 {
        return 1
    } else {
        return fibbonaci(n-1) + fibbonaci(n-2)
    }
}

pub fn main() {
    println!("Control Flow Baby");
    control_flow_ex();
    ternary();
    loopyland();
    whileloops();
    for_loops();

    for x in 0..10 {
        println!("Fibbonaci {} is {}", x, fibbonaci(x));
    }
}
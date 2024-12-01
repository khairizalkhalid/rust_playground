use std::io;

fn test_mutablity() {
    println!("Running test_mutability()");
    let mut x = 5;
    println!("Value of x is: {x}");
    x = 6;
    println!("Value of x is: {x}");
}

fn test_shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner value of x is: {x}");
    }
    println!("Outer value of x is: {x}")
}

fn test_numeric_operation() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = -5.0 / 3.0;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Truncated: {truncated}");
    println!("Remainder: {remainder}");
}

fn main() {
    println!("Select module to run:");
    println!("1) Mutability");
    println!("2) Shadowing");
    println!("3) Numeric Operations");
    println!("_____________");
    println!("Type 'quit' to, you've guessed it, Quit!");
    loop {
        let mut select = String::new();
        io::stdin()
            .read_line(&mut select)
            .expect("Unable to read input");
        if select.trim().eq_ignore_ascii_case("quit") {
            break;
        }
        if select.trim().eq_ignore_ascii_case("1") {
            test_mutablity();
        } else if select.trim().eq_ignore_ascii_case("2") {
            test_shadowing();
        } else if select.trim().eq_ignore_ascii_case("3") {
            test_numeric_operation();
        } else {
            println!("Invalid selection");
        }
    }
}

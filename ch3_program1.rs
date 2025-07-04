use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();

    // Read user input
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input to a number (i32)
    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    // Check if the number is positive, negative, or zero
    if number > 0 {
        println!("The number is positive.");
    } else if number < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }
}

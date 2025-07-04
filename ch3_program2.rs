use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();

    // Read input from the user
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Convert the input string to an integer (i32)
    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    // Check if the number is even or odd using modulus operator
    if number % 2 == 0 {
        println!("The number {} is even.", number);
    } else {
        println!("The number {} is odd.", number);
    }
}

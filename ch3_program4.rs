use std::io;

fn main() {
    // Get first number
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");

    // Get second number
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");

    // Get operator
    println!("Enter an operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read input");
    let operator = operator.trim();

    // Match expression for calculation
    match operator {
        "+" => println!("Result: {}", num1 + num2),
        "-" => println!("Result: {}", num1 - num2),
        "*" => println!("Result: {}", num1 * num2),
        "/" => {
            if num2 != 0.0 {
                println!("Result: {}", num1 / num2)
            } else {
                println!("Error: Division by zero is not allowed.");
            }
        }
        _ => println!("Invalid operator! Please use +, -, *, or /."),
    }
}

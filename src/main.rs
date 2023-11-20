use std::io;

fn main() {
    // Prompt the user to enter a number
    println!("Enter a number:");

    // Create a mutable string to store the user input
    let mut input = String::new();

    // Read the user input from the console
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into a floating-point number
    let number: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

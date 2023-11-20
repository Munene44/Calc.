use std::io;

fn main() {
    
    println!("Enter a number:");

    
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };
    let square = number * number;
    println!("The square of {} is: {}", number, square);
}
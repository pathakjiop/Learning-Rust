use std::io;

// Function to perform addition
fn add(x: f64, y: f64) -> f64 {
    x + y
}

// Function to perform subtraction
fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

// Function to perform multiplication
fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

// Function to perform division
fn divide(x: f64, y: f64) -> f64 {
    if y != 0.0 {
        x / y
    } else {
        panic!("Division by zero!")
    }
}

fn main() {
    println!("Simple Calculator");

    loop {
        println!("Choose an operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if choice == 5 {
            println!("Exiting...");
            break;
        }

        println!("Enter first number:");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        let num1: f64 = num1.trim().parse().expect("Please enter a valid number!");

        println!("Enter second number:");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");
        let num2: f64 = num2.trim().parse().expect("Please enter a valid number!");

        match choice {
            1 => println!("Result: {}", add(num1, num2)),
            2 => println!("Result: {}", subtract(num1, num2)),
            3 => println!("Result: {}", multiply(num1, num2)),
            4 => println!("Result: {}", divide(num1, num2)),
            _ => println!("Invalid choice!"),
        }
    }
}

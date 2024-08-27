use std::io;

pub enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero");
                0.0
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut input2 = String::new();
    println!("Enter the first number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Enter the operation (+, -, *, /): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let operator = input.trim();

    input2.clear();
    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");

    let operation = match operator {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result = calculate(operation);
    println!("The result is: {}", result);
}

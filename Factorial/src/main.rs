use std::io::{self, Write};

fn main() {
    print!("Enter a number to calculate its factorial: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u32 = input.trim().parse().unwrap();

    println!("Factorial of {} is {}", number, factorial(number));
}

fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}
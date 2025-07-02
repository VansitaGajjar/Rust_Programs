use std::io::{self, Write};

fn main() {
    print!("Enter the number to check if it is Armstrong number or not (Only 3 digit number): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u32 = input.trim().parse().unwrap();

    if is_armstrong(number) {
        println!("{} is an Armstrong number", number);
    } else {
        println!("{} is not an Armstrong number", number);
    }
}

fn is_armstrong(number: u32) -> bool {
    let mut sum = 0;
    let mut temp = number;
    while temp != 0 {
        let digit = temp % 10;
        sum += digit * digit * digit;
        temp /= 10;
    }
    sum == number
}
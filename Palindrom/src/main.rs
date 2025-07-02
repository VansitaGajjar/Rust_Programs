use std::io;

fn main() {
    println!("Palindrome Checker");

    println!("Enter a word:");

    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim().to_lowercase();
    let reversed: String = input.chars().rev().collect();

    if input == reversed {
        println!("{} is a palindrome", input);
    } else {
        println!("{} is not a palindrome.", input);
    }
}


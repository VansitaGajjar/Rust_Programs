use std::io::{self, Write};

fn main() {
    print!("Enter a word or sentence to reverse it: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // let reversed = input.trim().chars().rev().collect::<String>();
    // println!("Reversed: {}", reversed);

    let mut reversed = String::new();
    for c in input.trim().chars().rev() {
        reversed.push(c);
    }
    println!("Reversed: {}", reversed);
}

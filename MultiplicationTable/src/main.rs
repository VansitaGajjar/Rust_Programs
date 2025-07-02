use std::io;

fn main() {
    println!("Enter a number to generate multiplication table");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u32 = input.trim().parse().unwrap();

    for i in 1..=10 {
        println!("{} * {} = {}", number, i, number * i);
    }
}

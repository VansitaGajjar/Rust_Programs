use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number: ");

    io::stdin().read_line(&mut input).unwrap();

    let x: i8 = input.trim().parse().unwrap();
    println!("X is {}", x);
    
    let y: i32 = x as i32 + 5;
    println!("{} + 5 = {}", x, y);
    println!("Checking if {} is even or odd", y);

    if y % 2 == 0 {
        println!("{} is even", y);
    } else {
        println!("{} is odd", y);
    }
}
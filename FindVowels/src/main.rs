use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a string: ");

    io::stdin().read_line(&mut input).unwrap();

    println!("You entered: {}", input);
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;

    for c in input.chars() {
        if vowels.contains(&c.to_ascii_lowercase()) {
            println!("{} is a vowel", c);
            count += 1;
        }
    }

    println!("Number of vowels: {}", count);
}

use std::io::{self, Write};

fn main() {
    print!("Enter a number: ");

    io::stdout().flush().unwrap(); 

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    
    let num: u32 = num.trim().parse().expect("Please type a number!");
    print!("Fibonnaci series: ");
    
    fibonnaci_series(num);
}

fn fibonnaci_series(n: u32) {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        print!("{} ", a);
        let c = a + b;
        a = b;
        b = c;
    }
    println!();
}
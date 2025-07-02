use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut z = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut x).unwrap();

    println!("Enter the second number: ");
    io::stdin().read_line(&mut y).unwrap();

    println!("Enter the third number: ");
    io::stdin().read_line(&mut z).unwrap();

    let x1:i32 = x.trim().parse().unwrap();
    let y1:i32 = y.trim().parse().unwrap();
    let z1:i32 = z.trim().parse().unwrap();

    if x1 > y1 && x1 > z1 && x1 != y1 && x1 != z1 {
        println!("The largest number is {}", x1);
    } else if y1 > x1 && y1 > z1 && y1 != x1 && y1 != z1 {
        println!("The largest number is {}", y1);
    } else if z1 > x1 && z1 > y1 && z1 != x1 && z1 != y1 {
        println!("The largest number is {}", z1);
    } else if x1 == y1 && x1 == z1 {
        println!("All numbers are equal");
    } else if x1 == y1 {
        println!("First and second number is equal and largest");
    } else if x1 == z1 {
        println!("First and third number is equal and largest");
    } else {
        println!("Second and third number is equal and largest");
    }
}

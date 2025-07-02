use std::io;

fn main() {
    loop {
        println!("---------------------------------------------------------------");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");
        println!("---------------------------------------------------------------");

        let mut input = String::new();
        println!("Enter your choice: ");

        io::stdin().read_line(&mut input).unwrap();

        let choice: i32 = input.trim().parse().unwrap();

        println!("---------------------------------------------------------------");
        // if choice == 5 {
        //     println!("Have a good day!");
        //     println!("---------------------------------------------------------------");
        //     break;
        // }

        let mut num1:i32 = 0;
        let mut num2:i32 = 0;

        if choice >=1 && choice <= 4 {
            let mut a = String::new();
            println!("Enter the first number: ");
            io::stdin().read_line(&mut a).unwrap();
            let n1: i32 = a.trim().parse().unwrap();
            num1 = n1;

            let mut b = String::new();
            println!("Enter the second number: ");
            io::stdin().read_line(&mut b).unwrap();
            let n2: i32 = b.trim().parse().unwrap();
            num2 = n2;
        }

        match choice {
            1 => {
                println!("---------------------------------------------------------------");
                println!("{} + {} = {}", num1, num2, num1 + num2);
            }
            2 => {
                if num2 < num1 {
                    println!("---------------------------------------------------------------");
                    println!("{} - {} = {}", num1, num2, num1 - num2);
                } else {
                    println!("---------------------------------------------------------------");
                    println!("Num 1 must be greater than num 2");
                }
            }
            3 => {
                println!("---------------------------------------------------------------");
                println!("{} * {} = {}", num1, num2, num1 * num2);
            }
            4 => {
                if num1 == 0 || num2 == 0 {
                    println!("---------------------------------------------------------------");
                    println!("Cannot divide by 0");
                } else if num1 < num2 {
                    println!("---------------------------------------------------------------");
                    println!("Num 1 must be greater than num 2");
                } else {
                    println!("---------------------------------------------------------------");
                    println!("{} / {} = {}", num1, num2, num1 / num2);
                }
            }
            5 => {
                println!("---------------------------------------------------------------");
                println!("Have a good day!");
                println!("---------------------------------------------------------------");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}

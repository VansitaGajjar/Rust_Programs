use std::io;

fn main() {
    println!("Enter first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).unwrap();
    let number1: i64 = num1.trim().parse().unwrap();

    println!("Enter second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();
    let number2: i64 = num2.trim().parse().unwrap();

    println!(
        "Getting the self dividing numbers between {} to {}",
        number1, number2
    );
    println!("Self dividing numbers: ");
    get_self_dividing_numbers(number1, number2);
    println!();
}

fn get_self_dividing_numbers(num1: i64, num2: i64) {
    for curr_num in num1..=num2 {
        let mut local_var: i64 = curr_num;
        let mut rem: i64 = curr_num % 10;
        let mut num_left: i64;
        let mut temp: i64 = 1;

        while temp != 0 {
            if rem != 0 {
                if curr_num % rem == 0 {
                    num_left = local_var / 10;
                    rem = num_left % 10;
                    local_var = num_left;

                    if num_left == 0 {
                        print!("{} ", curr_num);
                        temp = 0
                    }
                } else {
                    temp = 0;
                }
            } else {
                temp = 0;
            }
        }
    }
}


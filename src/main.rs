use std::io;

fn main() {
    println!("Welcome to Simple Cli-Calculator!!!");

    let mut num1 = String::new();
    let mut num2  = String::new();
    let mut operator  = String::new();

    // Take input!
    println!("Enter first number: ");
    io::stdin().read_line(&mut num1).expect("Failed to readline!!");

    println!("Enter Second number: ");
    io::stdin().read_line(&mut num2).expect("Failed to readline!!");

    println!("What you wanna do(+,-,*,/):");
    io::stdin().read_line(&mut operator).expect("Failed to read line!!!");

    // Convert it into integers
    let a:u32 = num1.trim().parse().unwrap();
    let b:u32 = num2.trim().parse().unwrap();

    let op = operator.trim();

    match op {
    "+" => println!("{} + {} = {}", a,b,a+b),
    "-" => println!("{} - {} = {}", a,b,a-b),
    "*" => println!("{} * {} = {}", a,b,a*b),
    "/" => {
        if b == 0 {
            println!("Error: Devided by zero!!");
        } else{
            println!("{} / {} = {}", a,b,a/b);
        }
    }
    _ => println!("Error: Invalid opretor!"),
    }

}

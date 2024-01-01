//Generate nth Fibonacci number
use std::io;

fn main() {
    loop {
        println!("=====================================");
        println!("Enter value of n: ");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Please enter valid number");

        let n: u32 = n.trim().parse().unwrap();

        println!("{}th fibonacci number is {}", n, calculate_fibonacci(n));
    }
}

fn calculate_fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 0;
    } 
    if n == 2 {
        return 1;
    }

    calculate_fibonacci(n-1) + calculate_fibonacci(n-2)
}

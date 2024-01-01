//Fahrenheit to Celsius and vice versa program
use std::io;

fn main() {
    loop {
        println!("=========================================================");
        println!("\nFahrenheit->F, Celsius->C");
        println!("Convert F to C (Enter y) and Convert C to F (Enter n)");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Please enter valid choice");

        if choice.trim() == "y" {
            println!("Enter Fahrenheit- ");

            let mut f = String::new();

            io::stdin()
                .read_line(&mut f)
                .expect("Please enter valid number");

            let f: f32 = f.trim().parse().unwrap();

            println!("Resultant Celsius- {}", calculate_celsius(f));
        } else if choice.trim() == "n" {
            println!("Enter Celsius- ");

            let mut c = String::new();

            io::stdin()
                .read_line(&mut c)
                .expect("Please enter valid number");

            let c: f32 = c.trim().parse().unwrap();

            println!("Resultant Fahrenheit- {}", calculate_fahrenheit(c));
        }
    }
}

fn calculate_celsius(f: f32) -> f32 {
    (f-32.0)*(5.0/9.0)
}

fn calculate_fahrenheit(c: f32) -> f32 {
    ((c*9.0)/5.0)+32.0
}

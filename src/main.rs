use std::io;

fn calculate_area(a: u32, b: u32) -> f64 {
    (a * b) as f64
}

fn read_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a positive integer.");
                continue;
            }
        }
    }
}

fn main() {
    let width = read_input("Enter width:");
    let height = read_input("Enter height:");

    println!("Area is {:.2}", calculate_area(width, height));
}
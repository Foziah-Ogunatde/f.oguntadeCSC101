use std::io;
use std::f64::consts::PI;

fn main() {
    println!("Welcome to the Shape Calculator!");
    println!("Choose a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    // Read user's choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().unwrap_or(0);

    if choice == 1 {
        // Trapezium
        let height = get_number("Enter height:");
        let base1 = get_number("Enter base1:");
        let base2 = get_number("Enter base2:");
        let area = (height / 2.0) * (base1 + base2);
        println!("Area of Trapezium is: {:.2}", area);
    } else if choice == 2 {
        // Rhombus
        let d1 = get_number("Enter diagonal1:");
        let d2 = get_number("Enter diagonal2:");
        let area = 0.5 * d1 * d2;
        println!("Area of Rhombus is: {:.2}", area);
    } else if choice == 3 {
        // Parallelogram
        let base = get_number("Enter base:");
        let altitude = get_number("Enter altitude:");
        let area = base * altitude;
        println!("Area of Parallelogram is: {:.2}", area);
    } else if choice == 4 {
        // Cube
        let side = get_number("Enter side length:");
        let area = 6.0 * side * side;
        println!("Area of Cube is: {:.2}", area);
    } else if choice == 5 {
        // Cylinder
        let radius = get_number("Enter radius:");
        let height = get_number("Enter height:");
        let volume = PI * radius * radius * height;
        println!("Volume of Cylinder is: {:.2}", volume);
    } else {
        println!("Invalid choice. Please run the program again.");
    }
}

// Function to get a number from the user
fn get_number(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().unwrap_or(0.0)
}

use std::io;
use std::f64; // for handling floating-point numbers and sqrt

fn main() {
    // Step 1: Input values for a, b, and c
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    println!("Enter value for a:");
    io::stdin().read_line(&mut a_str).expect("Failed to read input");
    let a: f64 = a_str.trim().parse().expect("Please enter a valid number");

    println!("Enter value for b:");
    io::stdin().read_line(&mut b_str).expect("Failed to read input");
    let b: f64 = b_str.trim().parse().expect("Please enter a valid number");

    println!("Enter value for c:");
    io::stdin().read_line(&mut c_str).expect("Failed to read input");
    let c: f64 = c_str.trim().parse().expect("Please enter a valid number");

    // Step 2: Compute discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Step 3: Determine the nature of roots
    if discriminant > 0.0 {
        // Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.0 {
        // One real repeated root
        let root = -b / (2.0 * a);
        println!("The roots are real and equal:");
        println!("Root = {}", root);
    } else {
        // Complex roots
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);
        println!("The roots are complex and imaginary:");
        println!("Root 1 = {} + {}i", real_part, imaginary_part);
        println!("Root 2 = {} - {}i", real_part, imaginary_part);
    }
}



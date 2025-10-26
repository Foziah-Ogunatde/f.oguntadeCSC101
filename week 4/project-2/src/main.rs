use std::io;

fn main() {
    // Step 1: Collect user inputs
    let mut experience_str = String::new();
    let mut age_str = String::new();

    println!("Enter years of experience:");
    io::stdin().read_line(&mut experience_str).expect("Failed to read input");
    let experience: f64 = experience_str.trim().parse().expect("Enter a valid number");

    println!("Enter age of employee:");
    io::stdin().read_line(&mut age_str).expect("Failed to read input");
    let age: i32 = age_str.trim().parse().expect("Enter a valid age");

    // Step 2: Determine if experienced
    // Assuming 5 years or more means "experienced"
    let is_experienced = experience >= 5.0;

    // Step 3: Calculate incentive
    let incentive: f64;

    if is_experienced {
        if age >= 40 {
            incentive = 1_560_000.0;
        } else if age >= 30 {
            incentive = 1_480_000.0;
        } else if age < 28 {
            incentive = 1_300_000.0;
        } else {
            // For ages between 28 and 29, not explicitly stated — assume 1,300,000
            incentive = 1_300_000.0;
        }
    } else {
        incentive = 100_000.0;
    }

    // Step 4: Display result
    println!("----------------------------------");
    println!("Experience: {} years", experience);
    println!("Age: {} years", age);
    println!("Annual Incentive: ₦{}", incentive);
}

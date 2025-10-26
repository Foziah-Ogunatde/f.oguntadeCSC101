use std::io;

fn main() {
    println!("Enter lower bound:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lower_bound: i32 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter upper bound:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upper_bound: i32 = input2.trim().parse().expect("Please enter a valid number");

    println!("\nCounting from {} to {}:", lower_bound, upper_bound);
    for num in lower_bound..=upper_bound {
        println!("{}", num);
    }
}

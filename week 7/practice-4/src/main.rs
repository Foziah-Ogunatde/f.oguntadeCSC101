use std::io;
fn add(a: i32, b: i32){
    let sum = a+b;
    println!("sum of a and b = {}", sum);
}

fn main() {
   
    let mut input1 = String::new();
    println!("enter input for parameter a:");
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let a:i32 = input1.trim().parse().expect("invalid input");

    let mut input2 = String::new();
    println!("enter input for parameter b:");
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let b:i32 = input2.trim().parse().expect("invalid input");

    //call add function with arguements
    add(a, b);
}

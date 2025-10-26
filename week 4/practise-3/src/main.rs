// rust to claculate the area of a triangle for a given base and height
use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("enter base: ");
    io::stdin().read_line(&mut input1).expect("not a valid number");
    let base:f32 = input1.trim().parse().expect("not a valid number");

    println!("enter height: ");
    io::stdin().read_line(&mut input2).expect("not a valid number");
    let height:f32 = input2.trim().parse().expect("not a valid number");
   
    if base > 0.0 {
        let area:f32 = (base * height) / 2.0;
        println!(" area of a triangle: {}", area );
    }
  
}

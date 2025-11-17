fn main() {
    // using vec::new();
    let v : Vec<i64> = Vec::new();

    //printing the size of vector
    println!("\nThe length of Vec::new is {}",v.len());

    //using macro
    let v = vec!["Greece", "", "Brazil", "Kenya", "Sudan"];
  
    //printing the size of vector
    println!("\n The length of vec mcro is { }", v.len());

}

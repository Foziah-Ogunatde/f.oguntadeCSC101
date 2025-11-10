fn main() {
    let arr: [i32; 4] = [10, 20, 30, 40];

    println!("Array: {:?}", arr);
    println!("Size: {}", arr.len());

    for val in arr.iter() {
        println!("Value: {}", val);
    }
}
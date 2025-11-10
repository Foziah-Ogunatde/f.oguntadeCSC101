fn main() {
    //Array with explicit data type (i32) and fixed size
    let arr1: [i32; 4] = [10, 20, 30, 40];
    println!("\nArray with explicit data type:");
    println!("Array: {:?}", arr1);
    println!("Size: {}", arr1.len());

    //Array with implicit data type (f64 inferred) and fixed size
    let arr2 = [10.4, 20.7, 30.4, 40.9, 51.2, 72.2];
    println!("\nArray with implicit data type:");
    println!("Array: {:?}", arr2);
    println!("Size: {}", arr2.len());

    //Array initialized with default value (-1) repeated 8 times
    let arr3: [i32; 8] = [-1; 8];
    println!("\nArray with default values:");
    println!("Array: {:?}", arr3);
    println!("Size: {}", arr3.len());
}
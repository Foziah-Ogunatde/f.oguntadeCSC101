fn main() {
     //mutable array 
     let mut colors = ["red", "green", "orange", "magenta"];

    println!("\n original array {:?}", colors);

    // mutable slice
    let sliced_colors  = &mut colors[1..3];

    println!("First Slice = {:?}", sliced_colors);

    // change the value of the original slice at the first index 
    sliced_colors[1] = "violet";

    println!("Change Slice = {:?}", sliced_colors); 
    println!("New Array: {:?}", colors);
}

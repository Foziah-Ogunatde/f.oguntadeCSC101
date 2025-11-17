fn main() {
    let v = vec!['K','A','R','A','T','E']

    let mut input1 = String::new();

    println!("Enter an index value btw (0 - 5)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    //index is the non negtivevalue which is smller than the size of the vector
    let index:usize = input1.trim().parse().expect("Invalid input");

    //getting value at given index value
    let ch: char = v[index];

    print!("\n is the character for index"},",index)
}

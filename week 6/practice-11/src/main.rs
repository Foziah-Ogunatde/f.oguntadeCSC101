fn main() {
    let a: i32 = 2; // Bit representation: 10
    let b: i32 = 3; // Bit representation: 11

    let mut result: i32;

    // Bitwise AND
    result = a & b;
    println!("(a & b)  = {}", result);

    // Bitwise OR
    result = a | b;
    println!("(a | b)  = {}", result);

    // Bitwise XOR
    result = a ^ b;
    println!("(a ^ b)  = {}", result);

    // Bitwise NOT
    result = !b;
    println!("(!b)     = {}", result);

    // Left shift
    result = a << b;
    println!("(a << b) = {}", result);

    // Right shift
    result = a >> b;
    println!("(a >> b) = {}", result);
}

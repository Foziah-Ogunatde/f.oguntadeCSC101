fn main() {
    let p: f64 = 1000.0; // principal
    let r: f64 = 1.0;    // rate
    let t: f64 = 2.0;    // time

    // simple interest
    let si = (p * r * t) / 100.0;
    println!("Simple Interest is {}", si);

    let a = p + si;
    println!("Amount is {}", a);
}

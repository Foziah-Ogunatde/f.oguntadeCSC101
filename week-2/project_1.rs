fn main() {
    let p: f64 = 250_000_000.0; // principal
    let r: f64 = 10.0;          // rate
    let n: f64 = 5.0;           // time in years

    // formula: A = P * (1 + R/100)^n
    let a = p * (1.0 + (r / 100.0)).powf(n);

    println!("Amount is ₦{:.2}", a);

    // CI = A - P
    let ci = a - p;

    println!("Compound interest is ₦{:.2}", ci);
}

fn main() {
    let p: f64 = 510_000.0; // principal or initial value
    let r: f64 = 5.0;       // rate (%)
    let n: f64 = 3.0;       // time (years)

    // Formula for depreciation: A = P * (1 - r/100)^n
    let a = p * (1.0 - (r / 100.0)).powf(n);

    println!("Amount after depreciation is ₦{:.2}", a);

    // Depreciation = P - A
    let d = p - a;
    println!("Total depreciation is ₦{:.2}", d);
}

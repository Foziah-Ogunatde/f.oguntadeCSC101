fn main() {
    // Sales amounts for each item
    let tos = 450_000.0;

    let mac = 1_500_000.0;

    let hp = 750_000.0;

    let del = 2_850_000.0;

    let ace = 250_000.0;

    let amounts = [tos, mac, hp, del, ace];

    // sum
    let sum: f64 = amounts.iter().sum();

    //the average
    let average = sum / amounts.len() as f64;

    // Print

    println!("Total Sales: ₦{:.2}", sum);

    println!("Average Sales: ₦{:.2}", average);
}
use std::fs::File;
use std::io::Write;

fn main() {
    // Step 1: Define separate datasets as arrays
    let names = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Step 2: Merge datasets into a single vector of tuples
    let mut merged_data = Vec::new();
    for i in 0..names.len() {
        merged_data.push((names[i], ministries[i], zones[i]));
    }

    // Step 3: Display merged data
    println!("EFCC - Convicted Ministers Report\n");
    println!("{:<30} {:<20} {:<15}", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE");
    println!("{:-<70}", "");

    for (name, ministry, zone) in &merged_data {
        println!("{:<30} {:<20} {:<15}", name, ministry, zone);
    }

    // Step 4: Save merged data to a file
    let mut file = File::create("efcc_ministers_report.txt").expect("Could not create file");

    writeln!(file, "EFCC - Convicted Ministers Report\n").unwrap();
    writeln!(file, "{:<30} {:<20} {:<15}", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE").unwrap();
    writeln!(file, "{:-<70}", "").unwrap();

    for (name, ministry, zone) in &merged_data {
        writeln!(file, "{:<30} {:<20} {:<15}", name, ministry, zone).unwrap();
    }

    println!("Merged data saved to 'efcc_ministers_report.txt'");
}
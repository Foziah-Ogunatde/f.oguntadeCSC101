use std::fs::File;
use std::io::Write;

fn main() {
    // Step 1: Define drink categories using arrays
    let lager = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = ["Legend", "Turbo King", "Williams"];
    let non_alcoholic = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // Step 2: Create a file to save the data
    let mut file = File::create("nb_drinks.txt").expect("Could not create file");

    // Step 3: Write formatted data to the file
    writeln!(file, "Nigerian Breweries Plc - High-Quality Drink Categories\n").unwrap();
    writeln!(file, "{:<15} {:<15} {:<15}", "Lager", "Stout", "Non-Alcoholic").unwrap();
    writeln!(file, "{:-<45}", "").unwrap();

    for i in 0..6 {
        let lager_item = lager.get(i).unwrap_or(&"");
        let stout_item = stout.get(i).unwrap_or(&"");
        let non_item = non_alcoholic.get(i).unwrap_or(&"");
        writeln!(file, "{:<15} {:<15} {:<15}", lager_item, stout_item, non_item).unwrap();
    }

    // Step 4: Confirm success
    println!("Drink categories saved to 'nb_drinks.txt'");
}
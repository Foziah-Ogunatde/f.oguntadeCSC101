use std::io;

fn main() {
    let mut people: Vec<(&str, u32)> = Vec::new();

    println!("How many developers are being interviewed?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let total: usize = input.trim().parse().expect("Please enter a valid number");

    for i in 0..total {
        let mut name = String::new();
        let mut years = String::new();

        println!("Enter name of developer {}:", i + 1);
        io::stdin().read_line(&mut name).expect("Failed to read name");

        println!("Enter years of experience for {}:", name.trim());
        io::stdin().read_line(&mut years).expect("Failed to read experience");

        let experience: u32 = years.trim().parse().expect("Please enter a valid number");

        people.push((name.trim(), experience));
    }

    let mut most_experienced = people[0];
    for person in &people {
        if person.1 > most_experienced.1 {
            most_experienced = *person;
        }
    }

    println!(
        "\nThe most experienced developer is {} with {} years of experience.",
        most_experienced.0, most_experienced.1
    );
}
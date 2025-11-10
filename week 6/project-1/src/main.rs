use std::io;

fn main() {
    println!("Welcome to the Food Ordering System!");

    loop {
        println!("\n======== MENU ========");
        println!("P = Poundo Yam / Edinkaiko Soup - ₦3,200");
        println!("F = Fried Rice & Chicken         - ₦3,000");
        println!("A = Amala & Ewedu Soup           - ₦2,500");
        println!("E = Eba & Egusi Soup             - ₦2,000");
        println!("W = White Rice & Stew            - ₦2,500");
        println!("=========================");

        // Get food choice
        println!("Enter the code of your choice (P, F, A, E, W): ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim().to_uppercase();

        // Get quantity
        println!("Enter quantity: ");
        let mut quantity_input = String::new();
        io::stdin()
            .read_line(&mut quantity_input)
            .expect("Failed to read input");
        let quantity: i32 = quantity_input.trim().parse().expect("Please enter a valid number");

        // Determine price per item
        let price_per_item = match choice.as_str() {
            "P" => 3200,
            "F" => 3000,
            "A" => 2500,
            "E" => 2000,
            "W" => 2500,
            _ => {
                println!("Invalid item code!");
                continue;
            }
        };

        // Calculate total
        let total_cost = price_per_item * quantity;
        let final_cost: f64;

        if total_cost > 10_000 {
            final_cost = total_cost as f64 * 0.95; // 5% discount
            println!(
                "\nYou qualify for a 5% discount! \nOriginal Total: ₦{}\nDiscounted Total: ₦{:.2}",
                total_cost, final_cost
            );
        } else {
            final_cost = total_cost as f64;
            println!("\nTotal amount payable: ₦{:.2}", final_cost);
        }

        // Ask if user wants to order again
        println!("\nDo you want to make another order? (y/n): ");
        let mut again = String::new();
        io::stdin()
            .read_line(&mut again)
            .expect("Failed to read input");
        if again.trim().to_lowercase() != "y" {
            println!("\nThank you for your order! ");
            break;
        }
    }
}

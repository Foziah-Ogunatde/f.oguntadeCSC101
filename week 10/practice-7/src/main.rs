#[derive(Debug)]
struct Employee {
    name:String,
    company:String,
    age:u32
}

fn main() {
    let emp = Employee {
        name:String::from("Edidiong Jessica"),
        company:String::from("Ernst & Young"),
        age:29
    };
    println!("Name = {}", emp.name);
    println!("Company = {}", emp.company);
    println!("Age = {}", emp.age);
}

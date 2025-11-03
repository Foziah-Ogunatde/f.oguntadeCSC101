fn main() {
    let fullname = "chibudum john umeh";
    let department = "computer  science";
    let uni = "Pan Atlantic University";

    let mut school = "School of Science".to_string();
    // push string
    school.push_str("and Technology");

    println!("My name is: {}", fullname);
    //check length
    println!("The length my fullname is: {}",fullname.len());
    println!("I am a student of department: {}", department);
    println!("the name of my school is: {}", school);
    println!("the name of my University is: {}", uni);
}

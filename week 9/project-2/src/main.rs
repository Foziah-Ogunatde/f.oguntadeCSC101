use std::fs::File;
use std::io::Write;

fn main() {
    // Step 1: Define student records using a struct
    struct Student {
        name: &'static str,
        matric: &'static str,
        department: &'static str,
        level: u32,
    }

    // Step 2: Create a list of students using a vector
    let students = vec![
        Student { name: "Oluchi Mordi", matric: "ACC10211111", department: "Accounting", level: 300 },
        Student { name: "Adams Aliyu", matric: "ECO10110101", department: "Economics", level: 100 },
        Student { name: "Shania Bolade", matric: "CSC10328828", department: "Computer", level: 200 },
        Student { name: "Adekunle Gold", matric: "EEE11020202", department: "Electrical", level: 200 },
        Student { name: "Blanca Edemoh", matric: "MEE10202001", department: "Mechanical", level: 100 },
    ];

    // Step 3: Display student details on the screen
    println!("PAU SMIS - Student Records");
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    println!("{:-<60}", "");

    for student in &students {
        println!("{:<20} {:<15} {:<15} {:<5}", student.name, student.matric, student.department, student.level);
    }

    // Step 4: Save the same details into a file
    let mut file = File::create("student_records.txt").expect("Could not create file");

    writeln!(file, "PAU SMIS - Student Records\n").unwrap();
    writeln!(file, "{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level").unwrap();
    writeln!(file, "{:-<60}", "").unwrap();

    for student in &students {
        writeln!(file, "{:<20} {:<15} {:<15} {:<5}", student.name, student.matric, student.department, student.level).unwrap();
    }

    println!(" Student records saved to 'student_records.txt'");
}
use std::io;
//this struc contains a row in the APS table.
//each level has 5 fields

struct ApsRow {
    level: String,
    office_admin:String,
    academic: String,
    lawyer: String,
    teacher: String,
}


fn main() {
    // STEP 1: Build the APS Table using a vector
    let table = vec![
        ApsRow {
            level: "APS 1-2".to_string(),
            office_admin: "Intern".to_string(),
            academic: "".to_string(),
            lawyer: "Paralegal".to_string(),
            teacher: "Placement".to_string(),
        },
        ApsRow {
            level: "APS 3-5".to_string(),
            office_admin: "Administrator".to_string(),
            academic: "Research Assistant".to_string(),
            lawyer: "Junior Associate".to_string(),
            teacher: "Classroom Teacher".to_string(),
        },
        ApsRow {
            level: "APS 5-8".to_string(),
            office_admin: "Senior Administrator".to_string(),
            academic: "PhD Candidate".to_string(),
            lawyer: "Associate".to_string(),
            teacher: "Senior Teacher".to_string(),
        },
        ApsRow {
            level: "EL1 8-10".to_string(),
            office_admin: "Office Manager".to_string(),
            academic: "Post-Doc Researcher".to_string(),
            lawyer: "Senior Associate 1-2".to_string(),
            teacher: "Leading Teacher".to_string(),
        },
        ApsRow {
            level: "EL2 10-13".to_string(),
            office_admin: "Director".to_string(),
            academic: "Senior Lecturer".to_string(),
            lawyer: "Senior Associate 3-4".to_string(),
            teacher: "Deputy Principal".to_string(),
        },
        ApsRow {
            level: "SES".to_string(),
            office_admin: "CEO".to_string(),
            academic: "Dean".to_string(),
            lawyer: "Partner".to_string(),
            teacher: "Principal".to_string(),
        },
    ];

    // STEP 2:  My programme should ask the user for their role
    let mut role = String::new();
    println!("Enter your staff role (e.g., Associate, Administrator, Paralegal):");
    io::stdin().read_line(&mut role).expect("Failed to read role");
    let role = role.trim();

    // STEP 3: My programme should ask for years of experience
    let mut years_input = String::new();
    println!("Enter your years of experience:");
    io::stdin().read_line(&mut years_input).expect("Failed to read years");
    
    let years: u32 = years_input.trim().parse().expect("Please enter a valid number");

    // STEP 4: My programme should search for the role in the APS table
    let mut found_level = None;

    for row in &table {
        if row.office_admin.eq_ignore_ascii_case(role)
            || row.academic.eq_ignore_ascii_case(role)
            || row.lawyer.eq_ignore_ascii_case(role)
            || row.teacher.eq_ignore_ascii_case(role)
        {
            found_level = Some(&row.level);
            break;
        }
    }

    // STEP 5: If role given does not exist in the table
    if found_level.is_none() {
        println!("Role '{}' was not found in the APS system.", role);
        return;
    }

    let level = found_level.unwrap();
    println!("Your job role matches APS Level: {}", level);

    // STEP 6: I'll match experience to APS band
    if level == "APS 1-2" && years <= 2 {
        println!("Experience confirms: APS 1-2");
    } else if level == "APS 3-5" && (3..=5).contains(&years) {
        println!("Experience confirms: APS 3-5");
    } else if level == "APS 5-8" && (5..=8).contains(&years) {
        println!("Experience confirms: APS 5-8");
    } else if level == "EL1 8-10" && (8..=10).contains(&years) {
        println!("Experience confirms: EL1 8-10");
    } else if level == "EL2 10-13" && (10..=13).contains(&years) {
        println!("Experience confirms: EL2 10-13");
    } else if level == "SES" && years >= 13 {
        println!("Experience confirms: SES");
    } else {
        println!("Warning: Your years of experience do NOT match the APS range for this role.");
    }
}

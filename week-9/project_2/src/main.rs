use std::fs::File;
use std::io::Write;

fn main() {
    let student_names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_nums = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EE11020202", "MEE10202001"];
    let departments = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = vec![300, 100, 200, 200, 100];

    let mut file = File::create("student_management.txt").unwrap();

    writeln!(file, "STUDENT MANAGEMENT INFORMATION SYSTEM").unwrap();
    writeln!(file, "{:<20} {:<15} {:<15} {:<15}", "Name", "Matric Number", "Department", "Level").unwrap();

    for i in 0..student_names.len() {
        writeln!(file, "{:<20} {:<15} {:<15} {:<15}", student_names[i], matric_nums[i], departments[i], levels[i]).unwrap();
    }

    println!("Data written to file");
}

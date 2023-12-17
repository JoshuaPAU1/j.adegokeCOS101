struct Employee {
    company: String,
    ceo: String,
    age: u32,
}

fn main() {
    // Initialize a structure
    let emp1 = Employee {
        company: String::from("Microsoft Corporation"),
        ceo: String::from("Satya Nadella"),
        age: 56,
    };

    let emp2 = Employee {
        company: String::from("Google Inc."),
        ceo: String::from("Sundar Pichai"),
        age: 51,
    };

    // Pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}

// Fetch values of specific structure fields using the
// operator and print it to the console
fn display(emp: Employee) {
    println!("CEO is: {}, Company is: {}, Age is: {}", emp.ceo, emp.company, emp.age);
}

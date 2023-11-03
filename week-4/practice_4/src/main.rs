fn main() {
    let fullname = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "Science of Science".to_string();
    //push string
    school.push_str(" and Technology");

    println!("My name is {}", fullname);
    //check length
    println!("The length of my full name is: {}",fullname.len());
    println!("I am a student of {} department",department);
    println!("{}",school);
    println!("{}",uni);
}

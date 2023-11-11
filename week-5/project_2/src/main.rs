use std::io;

fn main() {
    println!("Enter your Age");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let age:i32 = input1.trim().parse().expect("Failed to input");

    println!("Are you experienced? (Enter true or false)");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let exp:bool = input2.trim().parse().expect("Failed to input");

    if age >= 40 && exp == true
    {
        println!("Your incentive is N1,560,000");
    } else if age >= 30 && age < 40 && exp == true 
    {
        println!("Your incentive is N1,480,000");
    } else if age < 28 && exp == true
    {
        println!("Your incentive is N1,300,000");
    } else
    {
        println!("Your incentive is N100,000");
    }
    
}

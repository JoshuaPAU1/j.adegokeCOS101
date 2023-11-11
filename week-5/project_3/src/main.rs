use std::io;

fn main() {
    println!("Input the letters beside the meals to purchase the meal");
    println!("Menu");
    println!("P = Poundo Yam/Edinkaiko Soup  - N3,200");
    println!("F = Fried Rice and Chicken  - N3,000");
    println!("A = Amala and Ewedu Soup  - N2,500");
    println!("E = Eba and Egusi Soup  - N2,000");
    println!("W = White Rice and Stew  - N2,500");

    let p = 3200;
    let f = 3000;
    let a = 2500;
    let e = 2000;
    let w = 2500;


    let mut total_cost = String::new();


    println!("Enter the letter here:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid value");

    println!("Please input your prefered quantity: ");
    let mut quant = String::new();
    io::stdin().read_line(&mut quant).expect("Invalid value");
    let quantity:i32 = quant.trim().parse().expect("Failed to input");

        if input.trim() == "P" {
        let total_cost = quantity * p;
        let discounted_cost = if total_cost > 10000 { total_cost - (total_cost * 5 / 100) } else { total_cost };
        println!("Your charge is N{}", discounted_cost);
    } else if input.trim() == "F" {
        let total_cost = quantity * f;
        let discounted_cost = if total_cost > 10000 { total_cost - (total_cost * 5 / 100) } else { total_cost };
        println!("Your charge is N{}", discounted_cost);
    } else if input.trim() == "A" {
        let total_cost = quantity * a;
        let discounted_cost = if total_cost > 10000 { total_cost - (total_cost * 5 / 100) } else { total_cost };
        println!("Your charge is N{}", discounted_cost);
    } else if input.trim() == "E" {
        let total_cost = quantity * e;
        let discounted_cost = if total_cost > 10000 { total_cost - (total_cost * 5 / 100) } else { total_cost };
        println!("Your charge is N{}", discounted_cost);
    } else if input.trim() == "W" {
        let total_cost = quantity * w;
        let discounted_cost = if total_cost > 10000 { total_cost - (total_cost * 5 / 100) } else { total_cost };
        println!("Your charge is N{}", discounted_cost);
    } else {
        println!("Invalid input. Please select a valid meal.");
}
}


use std::io;

fn main() {

    let mut researcher_counter = 0;

    while researcher_counter < 500 {
    println!("Please enter the name of the published papers");
    let mut name_pub = String::new();
    io::stdin().read_line(&mut name_pub).expect("Wrong value");

    println!("Please input the number of papers published");
    let mut num_pub = String::new();
    io::stdin().read_line(&mut num_pub).expect("Wrong value");
    let num:i32 = num_pub.trim().parse().expect("Wrong value");

    if num >= 3 && num <= 5
    {
        println!("Name of published papers: {}",name_pub);
        println!("The incentive is N500,000");
    } else if num > 5 && num < 10
    {
        println!("Name of published papers: {}",name_pub);
        print!("The incentive is N800,000");
    } else if num > 10
    {
        println!("Name of published papers: {}",name_pub);
        println!("The incentive is N1,000,000");
    } else
    {
        println!("Name of published papers: {}",name_pub);
        println!("The incentive is N100,000");
    }
    researcher_counter +=1;
    }
    


}

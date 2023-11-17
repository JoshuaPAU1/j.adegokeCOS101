use std::io;

fn main() {
    let mut eligible_count = 0;

    while eligible_count < 150 {

        println!("Please enter your name");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Wrong value");

        println!("Please enter your correct email");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Wrong value");

        println!("Please enter your department");
        let mut department = String::new();
        io::stdin().read_line(&mut department).expect("Wrong value");

        println!("Please input your State of origin");
        let mut SOG = String::new();
        io::stdin().read_line(&mut SOG).expect("Wrong value");

        println!("Please enter your status as a class representative:\nEnter Y for yes and N for no");
        let mut rep = String::new();
        io::stdin().read_line(&mut rep).expect("Wrong value");

        println!("Please enter your current CGPA");
        let mut gp = String::new();
        io::stdin().read_line(&mut gp).expect("Wrong value");
        let gpa:f64 = gp.trim().parse().expect("Wrong value");

        println!("Please enter your current level. Choose between (100,200,300,400,500)");
        let mut lv = String::new();
        io::stdin().read_line(&mut lv).expect("Wrong value");
        let lvl:i32 = lv.trim().parse().expect("Wrong value");

        if rep.trim() == "Y" && gpa > 4.0 && lvl == 200 || lvl == 300 || lvl == 400 || lvl == 500
        {
            println!("You can vote");
            println!("Name: {}",name);
            println!("Email: {}",email);
            println!("Department: {}",department);
            println!("State of Origin: {}",SOG);
            eligible_count +=1;
        } else 
        {
            println!("Sorry, you are not eligible to vote");
        }
    }
}

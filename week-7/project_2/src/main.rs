use std::io;

fn main() {
    println!("Please input your number of siblings:");
    let mut num_of_sib = String::new();
    io::stdin().read_line(&mut num_of_sib).expect("Wrong value");
    let num_of_sib:i32 = num_of_sib.trim().parse().expect("Wrong value");

    for i in 0..num_of_sib {
        println!("Enter the details for sibling #{}",i+1);
        println!("Enter name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Invalid input");

        println!("Enter Age of sibling:");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Invalid input");
        let age:i32 = age.trim().parse().expect("Wrong value");

        if age > 18 
        {
            println!("Is sibling married? 'yes' or 'no'");
            let mut marr_status = String::new();
            io::stdin().read_line(&mut marr_status).expect("Wrong value");

            if marr_status.trim() == "no" {
                println!("Is sibling a 'student' or 'worker'?");
                let mut single_status = String::new();
                io::stdin().read_line(&mut single_status).expect("Invalid answer");

                if single_status == "student"
                {
                    println!("Please enter your university:");
                    let mut uni = String::new();
                    io::stdin().read_line(&mut uni).expect("Invalid answer");

                    println!("Enter course of study:");
                    let mut course_study = String::new();
                    io::stdin().read_line(&mut course_study).expect("Invalid answer");

                    println!("Sibling #{}:", i + 1);
                    println!("Name: {}", name.trim());
                    println!("Age: {}", age);
                    println!("Married: No");
                    println!("Occupation: {}",single_status.trim());
                    println!("University: {}", uni.trim());
                    println!("Course: {}", course_study.trim());


                } else
                {
                    println!("Sibling #{}:", i + 1);
                    println!("Name: {}", name.trim());
                    println!("Age: {}", age);
                    println!("Married: {}",marr_status.trim());
                    println!("Occupation: {}",single_status.trim());
                }
            } 
            if marr_status.trim() == "yes"
            {
                println!("Do they have any offspring? ('yes' or 'no'");
                let mut offsprng = String::new();
                io::stdin().read_line(&mut offsprng).expect("Invalid answer");

                println!("What is their city of residence?");
                let mut city_of_res = String::new();
                io::stdin().read_line(&mut city_of_res).expect("Invalid answer");

                println!("Sibling #{}:", i + 1);
                println!("Name: {}", name.trim());
                println!("Age: {}", age);
                println!("Married: {}",marr_status.trim());
                println!("Has Offspring: {}",offsprng.trim());
                println!("City: {}", city_of_res.trim());
            }
        } else 
        {
            println!("WAEC status 'yes' or 'no'");
            let mut waec_stat = String::new();
            io::stdin().read_line(&mut waec_stat).expect("Invalid answer");

            if waec_stat.trim() == "yes"
            {
                println!("Input secondary school attended:");
                let mut sec_school = String::new();
                io::stdin().read_line(&mut sec_school).expect("Invalid answer");

                println!("Sibling #{}:", i + 1);
                println!("Name: {}", name.trim());
                println!("Age: {}", age);
                println!("Married: No");
                println!("WAEC Status: {}",waec_stat.trim());
                println!("Secondary School: {}", sec_school.trim());
            } else 
            {
                println!("Input current class level:");
                let mut class_lvl = String::new();
                io::stdin().read_line(&mut class_lvl).expect("Invalid answer");

                println!("Sibling #{}:", i + 1);
                println!("Name: {}", name.trim());
                println!("Age: {}", age);
                println!("Married: No",);
                println!("WAEC Status: {}",waec_stat.trim());
                println!("Current Class Level: {}", class_lvl.trim());
            }
        }
    }
}

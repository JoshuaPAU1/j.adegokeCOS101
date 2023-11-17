use std::io;

fn main() {
    println!("Please enter the multiplication table you would like to use?");
    let mut multiple = String::new();
    io::stdin().read_line(&mut multiple).expect("Wrong value");
    let multiple:i32 = multiple.trim().parse().expect("Wrong value");

    println!("Please enter the number you would like to stop at");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Wrong value");
    let n:i32 = n.trim().parse().expect("Wrong value");



    for m in 1..=n {
        let r = m * multiple;
        println!("{} * {} = {}",m,multiple,r);
    }
}

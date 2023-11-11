use std::io;

fn main() {
    println!("Enter the your value for a here:");
    let mut a1 = String::new();
    io::stdin().read_line(&mut a1).expect("Invalid value");
    let a:f64 = a1.trim().parse().expect("Failed to input");


    println!("Enter the your value for b here:");
    let mut b1 = String::new();
    io::stdin().read_line(&mut b1).expect("Invalid value");
    let b:f64 = b1.trim().parse().expect("Failed to input");

    println!("Enter the your value for c here:");
    let mut c1 = String::new();
    io::stdin().read_line(&mut c1).expect("Invalid value");
    let c:f64 = c1.trim().parse().expect("Failed to input");

    let discr = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let formula1 = ((-1.0 * b) + discr)/(2.0 * a);
    let formula2 = ((-1.0 * b) - discr)/(2.0 * a);

    if discr > 0.0
    {
        println!("The two roots are {} and {}",formula1,formula2);
    } else if discr == 0.0
    {
        println!("The one real root is {}",formula1);
    } else
    {
        println!("There is no real root");
    }
    
    


}

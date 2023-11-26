use std::io;

fn trapezium_formula() {
    println!("Input your height value:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Wrong input");
    let a:f64 = a.trim().parse().expect("Wrong value");

    println!("Input your first base value:");
    let mut b1 = String::new();
    io::stdin().read_line(&mut b1).expect("Wrong input");
    let b1:f64 = b1.trim().parse().expect("Wrong value");

    println!("Input your second base value:");
    let mut b2 = String::new();
    io::stdin().read_line(&mut b2).expect("Wrong input");
    let b2:f64 = b2.trim().parse().expect("Wrong value");

    let mut c = (a/2.0)*(b1 + b2);
    println!("The area of your trapezium is: {}",c)

}

fn rhombus_formula(){
    println!("Input your first diagonal value:");
    let mut b1 = String::new();
    io::stdin().read_line(&mut b1).expect("Wrong input");
    let b1:f64 = b1.trim().parse().expect("Wrong value");

    println!("Input your second diagonal value:");
    let mut b2 = String::new();
    io::stdin().read_line(&mut b2).expect("Wrong input");
    let b2:f64 = b2.trim().parse().expect("Wrong value");

    let mut c = 0.5 * b1 * b2;
    println!("The area of the rhombus is {}",c );

}

fn parallelogram_formula(){
    println!("Input your base value:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Wrong input");
    let b:f64 = b.trim().parse().expect("Wrong value");

    println!("Input your altitude value:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Wrong input");
    let a:f64 = a.trim().parse().expect("Wrong value");

    let mut c = b * a;
    println!("The area of the parallelogram is {}",c );

}

fn cube_formula(){
    println!("Input the length of the sides:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Wrong input");
    let b:f64 = b.trim().parse().expect("Wrong value");

    let mut c = 6.0 * (b).powf(2.0);
    println!("The area of the cube is {}",c );

}

fn cylinder_volume(){
    println!("Input the radius:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Wrong input");
    let b:f64 = b.trim().parse().expect("Wrong value");

    println!("Input the height:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Wrong input");
    let a:f64 = a.trim().parse().expect("Wrong value");

    let mut c = (22.0/7.0) * b * a;
    println!("The Volume of the Cylinder is {}",c );

}

fn main() {
    println!("Please select your preferred formula:");
    println!("A - Area of Trapezium\nB - Area of the rhombus\nC - Area of parallelogram\nD - Area of Cube\nE - Volume of Cylinder");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Wrong input");

    if option.trim() == "A"
    {
        trapezium_formula()
    } else if option.trim() == "B"
    {
        rhombus_formula()
    } else if option.trim() == "C"
    {
        parallelogram_formula()
    } else if option.trim() == "D"
    {
        cube_formula()
    } else if option.trim() == "E"
    {
        cylinder_volume()
    } else
    {
        println!("Invalid option");
    }
}

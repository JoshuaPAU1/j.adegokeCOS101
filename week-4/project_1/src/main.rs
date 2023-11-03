use std::io;

fn main() {
    //Assign the distances to variables before converting them to km
    let dista:f32 = 80.0;
    let distb:f32 = 120.0;
    let converter:f32 = 1.609;

    let converta = dista * converter;
    let convertb = distb * converter;

    let speeda = converta / 2.0;
    let speedb = convertb / 4.0;

    println!("The speed of a car travelling 80 miles at 2 hours is {}km/h",speeda);

    println!("And the speed of a car travelling 120 miles in 4 hours is {}km/h",speedb);

    //Users input
    println!("\nEnter your distance in miles.");
    let mut distance = String::new();
        io::stdin()
        .read_line(&mut distance)
        .expect("Failed to read input");
        let distance:f32 = distance.trim().parse().expect("Input not valid");
    println!("Your distance {} miles", distance);

    println!("\nEnter your time in hours.");
    let mut time = String::new();
        io::stdin()
        .read_line(&mut time)
        .expect("Failed to read input");
        let time:f32 = time.trim().parse().expect("Input not valid");
    println!("Your time is {} hours", time);

    let conv = distance * converter;
    let speed = conv/time;

    println!("Your speed is {}km/h",speed);

}

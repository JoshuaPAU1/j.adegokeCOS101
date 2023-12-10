use std::fs::File;
use std::io::Write;

fn main() {
    let lager = "\nOur Lager includes:\n33 Export\nDesperados\nGoldberg\nGuilder\nHeineken\nStar\n";
    let stout = "\nOur Stout includes:\nLegend\nTurbo\nWilliams\n";
    let non_alcoholic = "\nOur Non alcoholic includes:\nMaltina\nAmstel Malta\nMalta Gold\nFayrouz\n";

    let mut file = File::create("beers.txt").expect("create file");
    file.write_all("Welcome to Nigerian Breweries Limited. We have a rich portfolio of high-quality drinks: \n"
        .as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");
    println!("Data written to file");
}

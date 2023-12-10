use std::fs::File;
use std::io::Write;

fn main() {
    let names_of_commissioners = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "Justice", "Defence", "Power & Steel", "Petroleum"];
    let geogr_zone = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = File::create("EFCC.txt").unwrap();

    writeln!(file, "EFCC CONVICTED MINISTERS").unwrap();
    writeln!(file, "{:<30} {:<30} {:<20}", "Name of commissioner", "Ministry", "Geographical zone").unwrap();

    for i in 0..names_of_commissioners.len() {
        writeln!(file, "{:<30} {:<30} {:<20}", names_of_commissioners[i],ministry[i], geogr_zone[i]).unwrap();
    }

    println!("Data written to file");
}

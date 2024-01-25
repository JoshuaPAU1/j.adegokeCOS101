use std::io;
use std::io::Write;


fn main() {


    struct Company {
        name:String,
        username:String,
        password:String,
        time:u32,
        shares:f64,
        liabilities:f64,
    }
    let cadbury = Company {
        name:String::from("Cadbury Nigeria Plc"),
        username:String::from("cadb"),
        password:String::from("ca12"),
        time:1965,
        shares:15_000_000.00,
        liabilities:5_500_000.00,
    };
    let champion = Company {
        name:String::from("Champion Breweries Plc"),
        username:String::from("cham"),
        password:String::from("ch123"),
        time:1974,
        shares:25_000_000.00,
        liabilities:8_000_000.00,
    };
    let dangote = Company {
        name:String::from("Dangote Sugar Refinery Plc"),
        username:String::from("dang"),
        password:String::from("dang123"),
        time:1970,
        shares:18_000_000.00,
        liabilities:10_000_000.00,
    };
    let flour = Company {
        name:String::from("Flour Mills Nigeria Plc"),
        username:String::from("flou"),
        password:String::from("fl1234"),
        time:1960,
        shares:32_000_000.00,
        liabilities:4_000_000.00,
    };
    let nestle = Company {
        name:String::from("Nestle Nigeria Plc"),
        username:String::from("nest"),
        password:String::from("nes123"),
        time:1961,
        shares:8_000_000.00,
        liabilities:1_500_000.00,
    };
    let unilever = Company {
        name:String::from("Unilever Nigeria Plc"),
        username:String::from("unil"),
        password:String::from("uni123"),
        time:1923,
        shares:37_000_000.00,
        liabilities:11_000_000.00,
    };
    let honeywell = Company {
        name:String::from("Honeywell Nigeria Plc"),
        username:String::from("hone"),
        password:String::from("honey123"),
        time:1906,
        shares:34_000_000.00,
        liabilities:9_000_000.00,
    };
    let nigerian = Company {
        name:String::from("Nigeria Breweries Plc"),
        username:String::from("nige"),
        password:String::from("nige123"),
        time:1946,
        shares:30_000_000.00,
        liabilities:12_000_000.00,
    };
    let cadleverage = cadbury.shares - cadbury.liabilities;
    let cadpercentlev = (cadleverage/cadbury.shares) * 100.00;

    let cad_percent_lev_percent = (cadpercentlev/100.00) * cadleverage;
    let cad5percent = 0.05 * cad_percent_lev_percent;

    let champlev = champion.shares - champion.liabilities;
    let champercentlev = (champlev/champion.shares) * 100.00;

    let champercent_lev_5percent = (champercentlev/100.00) * champlev;
    let cham5percent = 0.05 * champercent_lev_5percent;

    let danglev = dangote.shares - dangote.liabilities;
    let dangpercentlev = (danglev/dangote.shares) * 100.00;

    let floulev = flour.shares - flour.liabilities;
    let flourpercentlev = (floulev/flour.shares) * 100.00;

    let floupercent_lev_5percent = (flourpercentlev/100.00) * floulev;
    let flou5percent = 0.05 * floupercent_lev_5percent;
    
    let neslev = nestle.shares - nestle.liabilities;
    let nestpercentlev = (neslev/nestle.shares) * 100.00;
    
    let nespercent_lev_5percent = (nestpercentlev/100.00) * neslev;
    let nes5percent = 0.05 * nespercent_lev_5percent;


    let unilev = unilever.shares - unilever.liabilities;
    let unilpercentlev = (unilev/unilever.shares) * 100.00;
    
    let honelev = honeywell.shares - honeywell.liabilities;
    let honepercentlev = (honelev/honeywell.shares) * 100.00;

    let honeypercent_lev_5percent = (honepercentlev/100.00) * honelev;
    let honey5percent = 0.05 * honeypercent_lev_5percent;
    
    let niglev = nigerian.shares - nigerian.liabilities;
    let nigepercentlev = (niglev/nigerian.shares) * 100.00;


    let mut username = String::new();
    println!("Please enter your username: ");
    io::stdin().read_line(&mut username).expect("Not a valid string");
    let username: String = username.trim().to_lowercase(); 

    let mut password = String::new();
    println!("Please enter your password: ");
    io::stdin().read_line(&mut password).expect("Not a valid string");

if username.len() < 3 {
        println!("Username is below 4 characters")
    } else {
        if username.trim() == cadbury.username && password.trim() == cadbury.password {
        println!("Valid info");
         let mut file = std::fs::File::create("company.txt").expect("Create failed");
    writeln!(file, "Company: {}", cadbury.name).expect("Write failed");
    writeln!(file, "Username: {}", cadbury.username).expect("Write failed");
    writeln!(file, "Password: {}", cadbury.password).expect("Write failed");
    writeln!(file, "Time: {}", cadbury.time).expect("Write failed");
    writeln!(file, "Shares: {}", cadbury.shares).expect("Write failed");
    writeln!(file, "Liabilities: {}", cadbury.liabilities).expect("Write failed");
    writeln!(file, "Percentage Leverage: {}%", cadpercentlev).expect("Write failed");
    writeln!(file, "Leverage for Liability below 10,000,000: {}", cad5percent).expect("Write failed");

    } else if username.trim() == champion.username && password.trim() == champion.password {
        println!("Valid info");
         let mut file = std::fs::File::create("company.txt").expect("Create failed");
    writeln!(file, "Company: {}", champion.name).expect("Write failed");
    writeln!(file, "Username: {}", champion.username).expect("Write failed");
    writeln!(file, "Password: {}", champion.password).expect("Write failed");
    writeln!(file, "Time: {}", champion.time).expect("Write failed");
    writeln!(file, "Shares: {}", champion.shares).expect("Write failed");
    writeln!(file, "Liabilities: {}", champion.liabilities).expect("Write failed");
    writeln!(file, "Percentage Leverage: {}%", champercentlev).expect("Write failed");
    writeln!(file, "Leverage for Liability below 10,000,000: {}", cham5percent).expect("Write failed");

    } else if username.trim() == dangote.username && password.trim() == dangote.password {
        println!("Valid info");
         let mut file = std::fs::File::create("company.txt").expect("Create failed");
    writeln!(file, "Company: {}", dangote.name).expect("Write failed");
    writeln!(file, "Username: {}", dangote.username).expect("Write failed");
    writeln!(file, "Password: {}", dangote.password).expect("Write failed");
    writeln!(file, "Time: {}", dangote.time).expect("Write failed");
    writeln!(file, "Shares: {}", dangote.shares).expect("Write failed");
    writeln!(file, "Liabilities: {}", dangote.liabilities).expect("Write failed");
    writeln!(file, "Percentage Leverage: {}%", dangpercentlev).expect("Write failed");

    } else if username.trim() == flour.username && password.trim() == flour.password {
        println!("Valid info");
         let mut file = std::fs::File::create("company.txt").expect("Create failed");
    writeln!(file, "Company: {}", flour.name).expect("Write failed");
    writeln!(file, "Username: {}", flour.username).expect("Write failed");
    writeln!(file, "Password: {}", flour.password).expect("Write failed");
    writeln!(file, "Time: {}", flour.time).expect("Write failed");
    writeln!(file, "Shares: {}", flour.shares).expect("Write failed");
    writeln!(file, "Liabilities: {}", flour.liabilities).expect("Write failed");
    writeln!(file, "Percentage Leverage: {}%", flourpercentlev).expect("Write failed");
    writeln!(file, "Leverage for Liability below 10,000,000: {}", flou5percent).expect("Write failed");

    } else if username.trim() == nestle.username && password.trim() == nestle.password {
        println!("Valid info");
        let mut file = std::fs::File::create("company.txt").expect("Create failed");
    writeln!(file, "Company: {}", nestle.name).expect("Write failed");
    writeln!(file, "Username: {}", nestle.username).expect("Write failed");
    writeln!(file, "Password: {}", nestle.password).expect("Write failed");
    writeln!(file, "Time: {}", nestle.time).expect("Write failed");
    writeln!(file, "Shares: {}", nestle.shares).expect("Write failed");
    writeln!(file, "Liabilities: {}", nestle.liabilities).expect("Write failed");
    writeln!(file, "Percentage Leverage: {}%", nestpercentlev).expect("Write failed");
    writeln!(file, "Leverage for Liability below 10,000,000: {}", nes5percent).expect("Write failed");

    } else if username.trim() == unilever.username && password.trim() == unilever.password{
        println!("Valid info");
        let mut file = std::fs::File::create("company.txt").expect("Create failed");
    writeln!(file, "Company: {}", unilever.name).expect("Write failed");
    writeln!(file, "Username: {}", unilever.username).expect("Write failed");
    writeln!(file, "Password: {}", unilever.password).expect("Write failed");
    writeln!(file, "Time: {}", unilever.time).expect("Write failed");
    writeln!(file, "Shares: {}", unilever.shares).expect("Write failed");
    writeln!(file, "Liabilities: {}", unilever.liabilities).expect("Write failed");
    writeln!(file, "Percentage Leverage: {}%", unilpercentlev).expect("Write failed");

    } else if username.trim() == honeywell.username && password.trim() == honeywell.password{
        println!("Valid info");
         let mut file = std::fs::File::create("company.txt").expect("Create failed");
    writeln!(file, "Company: {}", honeywell.name).expect("Write failed");
    writeln!(file, "Username: {}", honeywell.username).expect("Write failed");
    writeln!(file, "Password: {}", honeywell.password).expect("Write failed");
    writeln!(file, "Time: {}", honeywell.time).expect("Write failed");
    writeln!(file, "Shares: {}", honeywell.shares).expect("Write failed");
    writeln!(file, "Liabilities: {}", honeywell.liabilities).expect("Write failed");
    writeln!(file, "Percentage Leverage: {}%", honepercentlev).expect("Write failed");
    writeln!(file, "Leverage for Liability below 10,000,000: {}", honey5percent).expect("Write failed");


    } else if username.trim() == nigerian.username && password.trim() == nigerian.password{
        println!("Valid info");
        let mut file = std::fs::File::create("company.txt").expect("Create failed");
    writeln!(file, "Company: {}", nigerian.name).expect("Write failed");
    writeln!(file, "Username: {}", nigerian.username).expect("Write failed");
    writeln!(file, "Password: {}", nigerian.password).expect("Write failed");
    writeln!(file, "Time: {}", nigerian.time).expect("Write failed");
    writeln!(file, "Shares: {}", nigerian.shares).expect("Write failed");
    writeln!(file, "Liabilities: {}", nigerian.liabilities).expect("Write failed");
    writeln!(file, "Percentage Leverage: {}%", nigepercentlev).expect("Write failed");

    } else {
        println!("Invalid info");
    }        
}
}

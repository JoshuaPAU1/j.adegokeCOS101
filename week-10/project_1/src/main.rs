// Define a struct for representing laptop details
struct Laptop {
    brand: &'static str,
    price: u32,
}

// Implement a method for calculating the total cost of a given quantity
impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Define laptops with their respective prices
    let hp = Laptop {
        brand: "HP",
        price: 650000,
    };
    let ibm = Laptop {
        brand: "IBM",
        price: 755000,
    };
    let toshiba = Laptop {
        brand: "Toshiba",
        price: 550000,
    };
    let dell = Laptop {
        brand: "Dell",
        price: 850000,
    };

    // Number of laptops purchased from each brand
    let quantity_per_brand = 3;

    // Calculate the total cost for each brand
    let total_cost_hp = hp.total_cost(quantity_per_brand);
    let total_cost_ibm = ibm.total_cost(quantity_per_brand);
    let total_cost_toshiba = toshiba.total_cost(quantity_per_brand);
    let total_cost_dell = dell.total_cost(quantity_per_brand);

    // Calculate the overall total cost
    let overall_total_cost =
        total_cost_hp + total_cost_ibm + total_cost_toshiba + total_cost_dell;

    // Print the results
    println!("Total cost for HP laptops: {}", total_cost_hp);
    println!("Total cost for IBM laptops: {}", total_cost_ibm);
    println!("Total cost for Toshiba laptops: {}", total_cost_toshiba);
    println!("Total cost for Dell laptops: {}", total_cost_dell);
    println!("Overall total cost: {}", overall_total_cost);
}

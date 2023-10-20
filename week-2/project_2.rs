fn main(){
	let tosh:f64 = 450_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let dell:f64 = 2_850_000.0;
	let acer:f64 = 250_000.0;

	let toshiba = tosh * 2.0;
	let macbook = mac * 1.0;
	let hwp = hp * 3.0;
	let Dell = dell * 3.0;
	let Acer = acer * 1.0;

	let total = toshiba + macbook + hwp + Dell + Acer;

	let average = total / 10.0;

	println!("The total amount is {}",total);

	println!("The average of the sales is {}",average)
}
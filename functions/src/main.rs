use std::io;

fn five() -> i32 {
	5
}  
fn main() {
    another_function(40, 'm');
    let funf = five();
    println!(" five is {funf}");

	let mut decision = String::new();
	let mut value = String::new();

	println!("Which conversion would you like to do ?
	Type:
	1 for Farenheit to Celcius
	2 for Celcius to Farenheit");

	io::stdin()
		.read_line(&mut decision)
		.expect("Type 1 or 2 please ^^");

	let decision : u32 = decision.trim().parse().expect("Please give a valid input");
	  	

    if decision == 1 {
    	println!("Please input the Farenheit value: ");
    	io::stdin()
    		.read_line(&mut value)
    		.expect("error");
    	let value : f32 = value.trim().parse().expect("Please give a valid float input");
    	 
    	faren_to_celc(value);
    } 
    else if decision == 2 {
    	println!("Please input the Celcius value: ");
    	    	io::stdin()
    	    		.read_line(&mut value)
    	    		.expect("error");
    	let value : f32 = value.trim().parse().expect("Please give a valid float input");
    	
    	celc_to_faren(value);
    } 
    else {
    	println!("Please input either 1 or 2");
    }
}

fn another_function (value: u32, unit_label: char) {
	println!("The measurement is: {value} {unit_label}");
}

fn faren_to_celc (farenheit : f32) {

	let celcius = (farenheit - 32.0) / 1.8 ;
	
	println!("Farenheit = {farenheit} -> Celcius {celcius}" );
	
}

fn celc_to_faren (celcius : f32) {

	let farenheit = (celcius * 1.8) + 32.0 ;
	println!("Celcius = {celcius} -> Farenheit = {farenheit}");
}

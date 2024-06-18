fn main() {
	let mut resistance :f64 = 100.0;
 	let mut i = 0; 
    loop {
		resistance += resistance * (0.4 / 100.0);
		i += 1;
			if i == 25 {break};
				
    }
   let resistance2 = resistance + (resistance * (0.4 / 100.0));

    let r0 = 100.0;

    let deltav = (resistance2 / (r0 + resistance2)) - (resistance / (r0 + resistance)); //pt100

       let f = f64::powf(10.0, -6.0);
<<<<<<< HEAD
       
=======
    
>>>>>>> 04e335acf0e9b05b81b0b829f20c738702a8fb9e
       let g = f64::powf(10.0, -3.0);

       let y = f64::powf(25.0, 2.0);

       let x = f64::powf(26.0, 2.0);

	let r = 100.0 * (1.0 + (3.90802 * g * 25.0) - (0.580195 * f * y));
 	let r1 = 100.0 * (1.0 + (3.90802 * g * 26.0) - (0.580195 * f * x));

 	let deltav1 = (r1 / (r0 + r1)) - (r / (r0 + r)); 
	println! ("{deltav} {resistance} {resistance2}" ) ;       
}

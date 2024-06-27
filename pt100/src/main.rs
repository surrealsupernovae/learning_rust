fn main() {

    struct Values {
		temperature_1: f32,
		temperature_2: f32,
		current: f32,
		voltage: f32,
	} 

	impl Values {
 	fn thermal_resistance(&self) -> f32 {
 		let pow = &self.power();
 		let result = (self.temperature_1 - self.temperature_2) / pow;
 		result
 	}

 	fn power(&self) -> f32 {
 		self.current * self.voltage
 	}
 } 

 	let r_th_jc = Values {
 		 temperature_1: 73.834,
 		 temperature_2: 62.881,
 		 current: 1.36,
 		 voltage: 1.804,	
 	};

 	let r_th_cs = Values {
 	 	temperature_1: 62.881,
 	 	temperature_2: 53.302,
 	 	current: 1.36,
 	 	voltage: 1.804,
 	 };

 	 let r_th_sa = Values {
 	  	temperature_1: 53.302,
 	  	temperature_2: 25.0,
 	 	current: 1.36,
 	 	voltage: 1.804,	
 	 };

 	 
 	let rc_jc = Values {
 		 temperature_1: 72.872,
 		 temperature_2: 64.28,
 		 current: 1.36,
 		 voltage: 1.936,	
 	};

 	let rc_cs = Values {
 	 	temperature_1: 64.28,
 	 	temperature_2: 49.323,
 	 	current: 1.36,
 	 	voltage: 1.936,
 	 };

 	 let rc_sa = Values {
 	  	temperature_1: 49.323,
 	  	temperature_2: 25.0,
 	 	current: 1.36,
 	 	voltage: 1.936,	
 	 };
 	 
	let r_thjc = r_th_jc.thermal_resistance();
	let r_thcs = r_th_cs.thermal_resistance();	
 	let r_thsa = r_th_sa.thermal_resistance();

	let rc_thjc = rc_jc.thermal_resistance();
	let rc_thcs = rc_cs.thermal_resistance();	
 	let rc_thsa = rc_sa.thermal_resistance();
 	
 	println! ("r_jc {}, r_cs{}, r_sa {}", r_thjc, r_thcs, r_thsa);
 	println! ("rc_jc {}, rc_cs{}, rc_sa {}", rc_thjc, rc_thcs, rc_thsa); 
 	 
	let mut resistance :f64 = 100.0;
 	let mut i = 0; 
    loop {
		resistance += resistance * (0.4 / 100.0);
		i += 1;
			if i == 25 {break};
				
    }
   let resistance2 = resistance + (resistance * (0.4 / 100.0));

    let r0 = 100.0;

    let _deltav = (resistance2 / (r0 + resistance2)) - (resistance / (r0 + resistance)); //pt100

       let f = f64::powf(10.0, -6.0);
<<<<<<< HEAD
       
=======
    
>>>>>>> 04e335acf0e9b05b81b0b829f20c738702a8fb9e
       let g = f64::powf(10.0, -3.0);

       let y = f64::powf(25.0, 2.0);

       let x = f64::powf(26.0, 2.0);

	let r = 100.0 * (1.0 + (3.90802 * g * 25.0) - (0.580195 * f * y));
 	let r1 = 100.0 * (1.0 + (3.90802 * g * 26.0) - (0.580195 * f * x));

 	let _deltav1 = (r1 / (r0 + r1)) - (r / (r0 + r)); 
	//  println! ("{deltav} {resistance} {resistance2}" ) ;       
}


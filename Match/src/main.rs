fn main() {
	enum Coin {
		Penny,
		Nickel,
		Dime,
		Quarter,	  	
	}
	fn value_in_cents (coin: Coin) -> u8 {
		match coin {
			Coin::Penny => 1,
			Coin::Nickel => 5,
			Coin::Dime => 10,
			Coin::Quarter => 25,
		}
 	} 
 	fn plus_one (x: Option<i32>) -> Option<i32> {
 		match x {
 			None => None,
 			Some (i) => Some (i + 1),
 		}
 	}
	
 	let five = Some(5);
 	let _six = plus_one(five);
	let _none = plus_one(None);

	value_in_cents(Coin::Penny);
}

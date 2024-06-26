use std::collections::HashMap;
use std::io;

/*fn sort (vector : &Vec<i32>) -> Vec<i32> {
	for i in vector {
			//v.get(i);
					
	}
}
*/

fn main() {

let mut v : Vec<i32> = Vec::new(); 
 loop { 
	let mut entry = String::new();
	
	io::stdin().read_line(&mut entry).expect("you done goofed");
	
	let number: i32 = match entry.trim().parse() {
		Ok(num) =>  num,
		Err(_) => break,
	};
	v.push(number);
  }
}


/*plan :
make a vector, idk if it's possible to do it without itterations so we'll try that.  
*/ 

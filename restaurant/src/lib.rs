<<<<<<< HEAD
mod front_of_house {

   pub mod hosting {

    pub	fn add_to_waitlist() {}

    	fn add_seat_at_table() {}
	}
	
    mod serving {

    	fn take_order() {}
    	fn serve_order() {}
    	
    } 

}

pub fn eat_at_restaurant() {

	crate::front_of_house::hosting::add_to_waitlist();

	let mut meal = back_of_house::Breakfast::summer("vollkorn");

	meal.toast = String::from("white");
} 
mod back_of_house {
	pub enum Appetiser {
		Soup,
		Salad,
		Plancheta,
	}
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit : String,
		
	}
	impl Breakfast {
		pub fn summer (toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("bananas"),
				
			}
		}
	}
=======
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
>>>>>>> 04e335acf0e9b05b81b0b829f20c738702a8fb9e
}

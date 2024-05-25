use std::io;
use rand::Rng;


fn main() {
    println!("Guess a number!");
	let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("The number you guessed: {guess}"); 
    println!("Random number : {random_number}");
}

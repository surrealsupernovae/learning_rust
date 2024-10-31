use std::env;

fn main () {
	let args: Vec<String> = env::args().collect();

	let (query, path) = parse_config (&args);
	  
}

fn parse_config ( args: &[String]) -> (&str, &str) {
	let query = &args[1];
	let path = &args[2];

	(query,path)
	
}

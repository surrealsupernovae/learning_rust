use std::collections::HashMap;


fn main() {
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 20);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

score.entry(String::from("Blue")).or_insert(25);


for (key, value) in &scores {
	println!("{key} : {value}");
}



}

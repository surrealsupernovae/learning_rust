fn five() -> i32 {
	5
}  
fn main() {
    another_function(40, 'm');
    let funf = five();
    println!(" five is {funf}");
}

fn another_function (value: u32, unit_label: char) {
	println!("The measurement is: {value} {unit_label}");
}

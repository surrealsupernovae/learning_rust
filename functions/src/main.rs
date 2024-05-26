fn main() {
    another_function(40, 'm');
}

fn another_function (value: u32, unit_label: char) {
	println!("The measurement is: {value} {unit_label}");
}

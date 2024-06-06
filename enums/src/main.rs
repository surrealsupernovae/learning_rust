
fn main() {
	#[derive(Debug)]
	enum Ipaddrkind {
		V4(u8, u8, u8, u8),
		V6(String),
	}

	let _four = Ipaddrkind::V4;
	let _six = Ipaddrkind::V6;

	let home = Ipaddrkind::V4(126, 40, 0, 90);
	let huh = Ipaddrkind::V6(String::from("::1"));

	println!("{:?}", home);
	println!("{:?}", huh);	

	#[derive(Debug)]
	enum Message {
		Quit,
		Move{x: u32 , y:u32},
		Write(String),
		Changecolor(i32, i32, i32),
	}
	impl Message {
		fn call (&self) {
			println!("{:?}", self);
			
		}
	}
	let m = Message::Write(String::from("hello"));
	m.call();
}

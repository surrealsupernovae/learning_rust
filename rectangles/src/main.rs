#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	} 
	
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height  > other.height
	}
}

fn main() {

    let rect1 = Rectangle {
		width: 20,
		height: 10,
    };

    let rect2 = Rectangle {
    	width: 25,
    	height: 5, 
    };


    println! (
    	"The area of the rectangle is {}",
    	rect1.area()
	);
	
	println!("rectangle struct {:?}", rect1);
    
    println!("does rect2 fit in rect 1 {}", rect1.can_hold(&rect2));
}


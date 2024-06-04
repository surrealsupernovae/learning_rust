fn main() {
    let width1 = 10;
    let height1 = 5;

    println! (
    	"The area of the rectangle is {}",
    	area(width1, height1)

     );
    
}

fn area( w: u32, l: u32) -> u32 {
	w * l 
}

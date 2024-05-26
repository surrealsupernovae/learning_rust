fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut index = 0;
    let a = [1, 2, 3, 4, 5, 6];

    while index < 5 {
    	println!("value :{}", a[index]);
    	index += 1;
	}
    for element in a {
    	println!("{element}");
    }
    
}

extern crate rand;


use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main() {
    println!("Lisa macht die Ansagen!");
	let mut ansage = String::new();
	
	let secret_number = rand::thread_rng().gen_range(1, 101);
	
    println!("The secret number is: {}", secret_number);
	
	
	
	io::stdin().read_line(&mut ansage).expect("Lisa machte keine Ansagen =(");

	let ansage: u32 = ansage.trim().parse()
	        .expect("Please type a number!");
    
	match ansage.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}

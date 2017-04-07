use std::io;


fn main() {
    println!("Lisa macht die Ansagen!");
	let mut ansage = String::new();
	io::stdin().read_line(&mut ansage).expect("Lisa machte keine Ansagen =(");
	println!("Simon says: {}", ansage);
}


#[macro_use]
extern crate cluLog;

fn main() {
     init_clulog!(one);
	//Implementing LogDefault with one output stream.

	println!("Test out thread");
	eprintln!("Test out thread");
	
}


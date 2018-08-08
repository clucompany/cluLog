
#[macro_use]
extern crate clulog;


fn main() {
	init_clulog!();
	
	
	warn!("Warning");
	err!("This Machine Yrod :(");
	inf!("AAAAAA");
	
	unk!("START", "Unk {} {}", 23243, 21);
	
	
	println!("Println");
}


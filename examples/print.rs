
#[macro_use]
extern crate cluLog;


fn main() {
	init_cluLog!();	
	
	warn!("Warning");
	inf!("AAAAAA");
	
	unk!("START", "Unk {} {}", 23243, 21);
	unk!(?, "{} {}", 23243, 21);
	
	println!("Println");
}


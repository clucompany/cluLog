
#[macro_use]
extern crate clulog;


fn main() {
	/*let mut num = 1;
	for a in 0..400 {
		::std::thread::spawn(move || {
			loop {
				//init_clulog!();
				//init_clulog!(null);
				init_clulog!();
				init_clulog!(null);
				init_clulog!(none);
				init_clulog!();
				
				
				unk!("aa", "AAA {} {}", 23243, num);
				
				::std::thread::sleep_ms(num);
			}
		});
		num += 1;
		
		if num >= 15 {
			num = 1;
		}
	}*/
	init_clulog!(none);
	
	unk!("aa", "AAA {} {}", 23243, 21);
	println!("12");
	//::std::thread::park();
}


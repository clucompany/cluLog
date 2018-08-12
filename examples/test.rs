
#[macro_use]
extern crate cluLog;


fn main() {
	/*let mut num = 1;
	for a in 0..400 {
		::std::thread::spawn(move || {
			loop {
				//init_cluLog!();
				//init_cluLog!(null);
				init_cluLog!();
				init_cluLog!(null);
				init_cluLog!(none);
				init_cluLog!();
				
				
				unk!("aa", "AAA {} {}", 23243, num);
				
				::std::thread::sleep_ms(num);
			}
		});
		num += 1;
		
		if num >= 15 {
			num = 1;
		}
	}*/
	init_cluLog!(none);
	
	unk!("aa", "AAA {} {}", 23243, 21);
	println!("12");
	//::std::thread::park();
}



#[macro_use]
extern crate cluLog;


use std::fs::File;
use std::sync::Mutex;

fn main() {
	init_clulog!(one);
	//init_clulog!(one, Mutex::new(File::open("/tmp/out").unwrap()));
	//Implementing LogDefault with one output stream.

	/*let file = Mutex::new(File::create("/tmp/out").unwrap());
	let file2 = Mutex::new(File::create("/tmp/out2").unwrap());

	{
		let mut lock: LogSafeLock<MutexWriter<File>> = file._lock();
		let mut lock2: LogSafeLock<MutexWriter<File>> = file2._lock();

		let mut lock = lock.union(lock2);

		
		let _e = lock.write(b"Test");
	}*/
	let mut _file = Mutex::new(File::open("/tmp/out").unwrap());
	//let log = LogOneDefault::new(file);
	

	println!("Test out thread");
	eprintln!("Test out thread");

}


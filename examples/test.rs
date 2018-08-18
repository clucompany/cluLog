
#[macro_use]
extern crate cluLog;

use cluLog::log_lock::LogLockUnionConst;
use cluLog::log_lock::default::LogSafeLock;
use std::io::Write;
use cluLog::log::raw_lock::LogLockRawIO;
use cluLog::log::raw_lock::MutexWriter;
use cluLog::log_addition::union::LogUnionConst;
use cluLog::log::default::LogDefault;
use cluLog::log_addition::empty::default::LogEmpty;
use std::fs::File;
use std::sync::Mutex;

fn main() {
	init_clulog!(one);
	//init_clulog!(one, Mutex::new(File::open("/tmp/out").unwrap()));
	//Implementing LogDefault with one output stream.

	let file = Mutex::new(File::create("/tmp/out").unwrap());
	let file2 = Mutex::new(File::create("/tmp/out2").unwrap());

	{
		let mut lock: LogSafeLock<MutexWriter<File>> = file._lock();
		let mut lock2: LogSafeLock<MutexWriter<File>> = file2._lock();

		let mut lock = lock.union(lock2);

		
		let _e = lock.write(b"Test");
	}
	

	println!("Test out thread");
	eprintln!("Test out thread");

}


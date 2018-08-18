
#[macro_use]
extern crate cluLog;

use cluLog::log_addition::union::LogUnionConst;
use cluLog::log::default::LogDefault;
use cluLog::log_addition::empty::default::LogEmpty;
use std::fs::File;

fn main() {
	//init_clulog!(one, File::open("./out").unwrap());
	//Implementing LogDefault with one output stream.

	println!("Test out thread");
	eprintln!("Test out thread");

}


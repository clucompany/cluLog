
use std::io::Write;

pub mod default;
pub mod default_nf;


///Secure outflow blocking
#[allow(non_camel_case_types)]
pub trait LogLockIO<'a> {
	///Blocking threads with automatic cleaning
	fn lock_out(&'a self) -> Box<Write + 'a>;

	///Blocking threads with automatic cleaning
	fn lock_err(&'a self) -> Box<Write + 'a>;

	///Flow blocking without self-cleaning
	fn no_flush_lock_out(&'a self) -> Box<Write + 'a>;

	///Flow blocking without self-cleaning
	fn no_flush_lock_err(&'a self) -> Box<Write + 'a>;
}


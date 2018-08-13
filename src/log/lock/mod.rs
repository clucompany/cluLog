
use std::io::Write;
use std::ops::DerefMut;

pub mod default;
pub mod default_no_flush;




///Secure outflow blocking
#[allow(non_camel_case_types)]
pub trait LogLockIO<'a> {
	///Blocking threads with automatic cleaning
	fn lock_out<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>>;

	///Blocking threads with automatic cleaning
	fn lock_err<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>>;

	///Flow blocking without self-cleaning
	fn no_flush_lock_out<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>>;

	///Flow blocking without self-cleaning
	fn no_flush_lock_err<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>>;
}
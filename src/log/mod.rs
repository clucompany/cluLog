
#[macro_use]
pub mod empty;
#[macro_use]
pub mod kmsg;
#[macro_use]
pub mod std;
#[macro_use]
pub mod lock;
pub mod enable;

use std::ops::DerefMut;
use std::io::Write;
use std::fmt::Arguments;
use std::io;


#[allow(non_camel_case_types)]
pub trait cluLog: Send + Sync + cluLogIOLock {
	fn warning<'a>(&self, args: Arguments<'a>) -> io::Result<()>;
	
	fn info<'a>(&self, args: Arguments<'a>) -> io::Result<()>;
	
	fn error<'a>(&self, args: Arguments<'a>) -> io::Result<()>;
	
	fn panic<'a>(&self, args: Arguments<'a>) -> io::Result<()>;
	
	fn unknown<'a>(&self, name: &'a str, args: Arguments<'a>) -> io::Result<()>;
	
	fn print<'a>(&self, args: Arguments<'a>) -> io::Result<()>;
	fn eprint<'a>(&self, args: Arguments<'a>) -> io::Result<()>;
}

///Secure outflow blocking
#[allow(non_camel_case_types)]
pub trait cluLogIOLock: Send + Sync {
	///Blocking threads with automatic cleaning
	fn lock_out<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>>;

	///Blocking threads with automatic cleaning
	fn lock_err<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>>;

	///Flow blocking without self-cleaning
	fn no_flush_lock_out<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>>;

	///Flow blocking without self-cleaning
	fn no_flush_lock_err<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>>;
}

///Flush of output streams
#[allow(non_camel_case_types)]
pub trait cluLogFlushIO: Send + Sync {
	///Flush the output stream
	fn flush_out(&self) -> io::Result<()>;

	///Flush the err-output stream
	fn flush_err(&self) -> io::Result<()>;
	
	///Flush Out stream and Err stream
	fn flush(&self) -> io::Result<()> {
		if let Err(e) = self.flush_out() {
			return Err(e);
		}
		if let Err(e) = self.flush_err() {
			return Err(e);
		}

		Ok( () )
	}
}
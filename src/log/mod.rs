
pub mod raw_lock;
#[macro_use]
pub mod empty;
#[macro_use]
pub mod default;
#[macro_use]
pub mod lock;
pub mod enable;
pub mod union;

use std::ops::DerefMut;
use std::io::Write;
use std::fmt::Arguments;
use std::io;


#[allow(non_camel_case_types)]
pub trait cluLog<'a>: LogLockIO<'a> + LogFlushIO {
	fn warning<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn info<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn error<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn panic<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn unknown<'s>(&'a self, name: &'static str, args: Arguments<'s>) -> io::Result<()>;

	fn trace<'s>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()>;
	
	fn print<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;

	fn eprint<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
}

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



///Flush of output streams
#[allow(non_camel_case_types)]
pub trait LogFlushIO {
	///Flush the output stream
	fn flush_out(&mut self) -> io::Result<()>;

	///Flush the err-output stream
	fn flush_err(&mut self) -> io::Result<()>;
	
	///Flush Out stream and Err stream
	#[inline]
	fn flush(&mut self) -> io::Result<()> {
		if let Err(e) = self.flush_out() {
			return Err(e);
		}
		if let Err(e) = self.flush_err() {
			return Err(e);
		}

		Ok( () )
	}
}
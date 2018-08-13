
pub mod raw_lock;
#[macro_use]
pub mod default;
pub mod lock;
pub mod enable;

use log::lock::LogLockIO;
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
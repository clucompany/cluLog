
use crate::log_core::LogExtend;
use crate::log_core::LogStatic;
use crate::log_core::LogLockIO;
use crate::log_core::LogBase;
use crate::log_core::LogFlush;
use std::io::Write;
use std::fmt::Arguments;
use std::io;

#[derive(Debug)]
pub struct LogDefStaticLog;

impl LogDefStaticLog {
	#[inline(always)]
	pub const fn new() -> Self {
		LogDefStaticLog
	}
}

impl Clone for LogDefStaticLog {
	#[inline(always)]
	fn clone(&self) -> Self {
		LogDefStaticLog
	}
}


impl<'a> LogBase<'a> for LogDefStaticLog {
	#[inline(always)]
	fn warning<'l>(&'a self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn info<'l>(&'a self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn error<'l>(&'a self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		panic!("{}", args);
	}
	
	#[inline(always)]	
	fn unknown<'l>(&'a self, _name: &'static str, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}

	#[inline(always)]
	fn trace<'l>(&'a self, _line: u32, _pos: u32, _file: &'static str, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]	
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		io::stdout().write_fmt(args)
	}
	
	#[inline(always)]	
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		io::stderr().write_fmt(args)
	}
}

impl<'a> LogFlush<'a> for LogDefStaticLog {
	#[inline(always)]	
	fn flush_out(&self) -> io::Result<()> {
		io::stdout().flush()
	}
	
	#[inline(always)]
	fn flush_err(&self) -> io::Result<()> {
		io::stderr().flush()
	}
}

impl<'a> LogLockIO<'a> for LogDefStaticLog {
     #[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		Box::new(io::stdout())
	}

     #[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		Box::new(io::stderr())
	}
}


impl<'a> LogStatic<'a> for LogDefStaticLog {}
impl<'a> LogExtend<'a> for LogDefStaticLog {}

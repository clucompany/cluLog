
use crate::core::LogExtend;
use crate::core::LogStatic;
use crate::core::LogLockIO;
use crate::core::LogFlush;
use crate::core::LogBase;
use std::io::Write;
use cluExtIO::EmptyWrite;
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
	fn print<'l>(&'a self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]	
	fn eprint<'l>(&'a self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
}

impl<'a> LogFlush<'a> for LogDefStaticLog {
	#[inline(always)]	
	fn flush_out(&self) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn flush_err(&self) -> io::Result<()> {
		Ok( () )
	}
}

impl<'a> LogLockIO<'a> for LogDefStaticLog {
     #[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		EmptyWrite::new().into()
	}

     #[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		EmptyWrite::new().into()
	}
}


impl<'a> LogStatic<'a> for LogDefStaticLog {}
impl<'a> LogExtend<'a> for LogDefStaticLog {}

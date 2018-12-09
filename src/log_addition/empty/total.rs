

use crate::log_addition::LogEmptyConst;
use crate::log_core::LogBase;
use crate::log_core::LogStatic;
use crate::log_core::LogExtend;
use crate::log_core::LogLockIO;
use crate::log_core::LogFlush;

use std::io::Write;
use cluExtIO::EmptyWrite;
use std::fmt::Arguments;
use std::io;

#[derive(Debug)]
pub struct LogTotalEmpty;

impl LogTotalEmpty {
	#[inline(always)]
	pub fn new() -> Self {
		LogTotalEmpty
	}
}

impl LogEmptyConst for LogTotalEmpty {
	#[inline]
	fn empty() -> Self {
		Self::new()
	}
}

impl Clone for LogTotalEmpty {
	#[inline(always)]
	fn clone(&self) -> Self {
		LogTotalEmpty
	}
}


impl<'a> LogBase<'a> for LogTotalEmpty {
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

impl<'a> LogFlush<'a> for LogTotalEmpty {
	#[inline(always)]	
	fn flush_out(&self) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn flush_err(&self) -> io::Result<()> {
		Ok( () )
	}
}

impl<'a> LogLockIO<'a> for LogTotalEmpty {
     #[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		EmptyWrite::boxed()
	}

     #[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		EmptyWrite::boxed()
	}
}


impl<'a> LogStatic<'a> for LogTotalEmpty {}
impl<'a> LogExtend<'a> for LogTotalEmpty {}

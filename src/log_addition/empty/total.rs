

use log::LogStatic;
use log::LogExtend;
use log::LogBase;
use log_addition::union::LogUnionConst;
use log::lock::LogLockIO;
use log::LogFlushIO;
use std::io::Write;
use std::fmt::Arguments;
use std::io;
use log::lock::default::LogLock;
use log::lock::default_no_flush::LogLockNoFlush;
use log_addition::empty::LogEmptyConst;

#[derive(Debug)]
pub struct LogTotalEmpty;

impl LogTotalEmpty {
	pub fn new() -> Self {
		LogTotalEmpty
	}
}


impl<'a> LogBase<'a> for LogTotalEmpty {
	#[inline(always)]
	fn warning<'l>(&self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn info<'l>(&self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn error<'l>(&self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn panic<'l>(&self, args: Arguments<'l>) -> io::Result<()> {
		panic!("{}", args);
	}
	
	#[inline(always)]	
	fn unknown<'l>(&self, _name: &'static str, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}

	#[inline(always)]
	fn trace<'l>(&self, _line: u32, _pos: u32, _file: &'static str, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]	
	fn print<'l>(&self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]	
	fn eprint<'l>(&self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}
}

impl LogFlushIO for LogTotalEmpty {
	#[inline(always)]	
	fn flush_out(&mut self) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn flush_err(&mut self) -> io::Result<()> {
		Ok( () )
	}
}

impl<'a> LogLockIO<'a> for LogTotalEmpty {
	fn lock_out(&'a self) -> Box<Write + 'a> {
		LogLock::empty_boxed()
	}

	fn lock_err(&'a self) -> Box<Write + 'a> {
		LogLock::empty_boxed()
	}

	fn no_flush_lock_out(&'a self) -> Box<Write + 'a> {
		LogLockNoFlush::empty_boxed()
	}

	fn no_flush_lock_err(&'a self) -> Box<Write + 'a> {
		LogLockNoFlush::empty_boxed()
	}
}


impl<'a> LogUnionConst<'a> for LogTotalEmpty {}
impl<'a> LogStatic<'a> for LogTotalEmpty {}
impl<'a> LogExtend<'a> for LogTotalEmpty {}

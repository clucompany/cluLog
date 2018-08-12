

use log::union::LogUnion;
use log::LogFlushIO;
use log::LogLockIO;
use std::io::Write;
use std::ops::DerefMut;
use log::lock::cluLogLock;
use std::fmt::Arguments;
use log::cluLog;
use std::io;

#[derive(Debug)]
pub struct LogTotalEmpty;

impl LogTotalEmpty {
	pub fn new() -> Self {
		LogTotalEmpty
	}
	#[inline]
	pub fn union<'a>() -> LogUnion<'a, LogTotalEmpty, LogTotalEmpty> {
		LogUnion::total_empty()
	}
}


impl<'l> cluLog<'l> for LogTotalEmpty {
	#[inline(always)]
	fn warning<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn info<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn error<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn panic<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		panic!("{}", args);
	}
	
	#[inline(always)]	
	fn unknown<'a>(&self, _name: &'static str, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}

	#[inline(always)]
	fn trace<'a>(&self, _line: u32, _pos: u32, _file: &'static str, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]	
	fn print<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]	
	fn eprint<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
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
	fn lock_out<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>> {
		cluLogLock::empty_boxed()
	}
	
	fn lock_err<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>> {
		cluLogLock::empty_boxed()
	}

	fn no_flush_lock_out<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>> {
		cluLogLock::empty_boxed()
	}

	fn no_flush_lock_err<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>> {
		cluLogLock::empty_boxed()
	}
}

use log_addition::union::LogUnionConst;
use log_addition::union::default::LogUnion;
use log::lock::LogLockIO;
use log::LogFlushIO;
use std::ops::DerefMut;
use log::cluLog;
use std::fmt::Arguments;
use std::io::Write;
use std::io;
use log::lock::default::cluLogLock;
use log_addition::empty::LogEmptyConst;


#[derive(Debug)]
pub struct LogEmpty;

impl LogEmpty {
	pub fn new() -> Self {
		LogEmpty
	}

	#[inline]
	pub fn union<'a>() -> LogUnion<'a, LogEmpty, LogEmpty> {
		LogUnion::empty()
	}
}

impl<'l> cluLog<'l> for LogEmpty {
	#[inline(always)]
	fn warning<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}

	#[inline(always)]
	fn trace<'a>(&self, _line: u32, _pos: u32, _file: &'static str, _args: Arguments<'a>) -> io::Result<()> {
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
	
	fn print<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		io::stdout().write_fmt(args)
	}
	
	fn eprint<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		io::stderr().write_fmt(args)
	}
}

impl LogFlushIO for LogEmpty {
	#[inline(always)]	
	fn flush_out(&mut self) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn flush_err(&mut self) -> io::Result<()> {
		Ok( () )
	}
}


impl<'a> LogLockIO<'a> for LogEmpty {
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

impl<'a> LogUnionConst<'a> for LogEmpty {}
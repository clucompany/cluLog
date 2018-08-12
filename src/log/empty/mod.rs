
use log::union::LogUnion;
use log::cluLogFlushIO;
use log::cluLogIOLock;
use std::ops::DerefMut;
use log::cluLog;
use std::fmt::Arguments;
use std::io::Write;
use std::io;
use ::log::lock::cluLogLock;

pub mod write;
pub mod total;

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
	fn unknown<'a>(&self, _name: &'a str, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	fn print<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		io::stdout().write_fmt(args)
	}
	
	fn eprint<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		io::stderr().write_fmt(args)
	}
}

impl cluLogFlushIO for LogEmpty {
	#[inline(always)]	
	fn flush_out(&mut self) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn flush_err(&mut self) -> io::Result<()> {
		Ok( () )
	}
}


impl<'a> cluLogIOLock<'a> for LogEmpty {
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
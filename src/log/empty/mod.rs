
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

impl cluLog for LogEmpty {
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
	
	#[inline(always)]	
	fn print<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		io::stdout().write_fmt(args)
	}
	
	#[inline(always)]	
	fn eprint<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		io::stderr().write_fmt(args)
	}
}

impl cluLogFlushIO for LogEmpty {
	#[inline(always)]	
	fn flush_out(&self) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn flush_err(&self) -> io::Result<()> {
		Ok( () )
	}
}


impl cluLogIOLock for LogEmpty {
	#[inline]
	fn lock_out<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>> {
		cluLogLock::empty_boxed()
	}
	
	#[inline]
	fn lock_err<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>> {
		cluLogLock::empty_boxed()
	}

	#[inline]
	fn no_flush_lock_out<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>> {
		cluLogLock::empty_boxed()
	}

	#[inline]
	fn no_flush_lock_err<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>> {
		cluLogLock::empty_boxed()
	}
}
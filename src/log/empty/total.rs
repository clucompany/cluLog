

use log::cluLogFlushIO;
use log::cluLogIOLock;
use std::io::Write;
use std::ops::DerefMut;
use log::lock::cluLogLock;
use std::fmt::Arguments;
use log::cluLog;
use std::io;

#[derive(Debug)]
pub struct LogTotalEmpty;

impl cluLog for LogTotalEmpty {
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
	fn print<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]	
	fn eprint<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
}

impl cluLogFlushIO for LogTotalEmpty {
	#[inline(always)]	
	fn flush_out(&self) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn flush_err(&self) -> io::Result<()> {
		Ok( () )
	}
}

impl cluLogIOLock for LogTotalEmpty {
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
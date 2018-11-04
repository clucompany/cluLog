

use std::io::Write;
use log_core::LogBase;
use log_core::LogStatic;
use log_core::LogExtend;
use log_core::LogLockIO;
use log_core::LogFlush;
use std::fmt::Arguments;
use std::io;
use log_write::EmptyWrite;


#[derive(Debug, Clone)]
pub struct LogTotalEmpty;

impl LogTotalEmpty {
	#[inline(always)]
	pub fn new() -> Self {
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

impl LogFlush for LogTotalEmpty {
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
	fn no_flush_lock_out(&'a self) -> Box<Write + 'a> {
		EmptyWrite::boxed()
	}

	fn no_flush_lock_err(&'a self) -> Box<Write + 'a> {
		EmptyWrite::boxed()
	}
}


//impl<'a> LogUnionConst<'a> for LogTotalEmpty {}
impl<'a> LogStatic<'a> for LogTotalEmpty {}
impl<'a> LogExtend<'a> for LogTotalEmpty {}

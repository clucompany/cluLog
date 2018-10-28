

use log_core::LogBase;
use log_core::LogStatic;
use log_core::LogExtend;
use log_core::LogLockIO;
use log_core::LogFlush;
use log_lock::LogSafeLock;
use log_addition::union::LogUnionConst;
use std::fmt::Arguments;
use std::io;
use log_addition::empty::LogEmptyConst;
use log_lock::LogSafeWriteLock;
use log_lock::LogSafeWriteNFLock;

#[derive(Debug, Clone)]
pub struct LogTotalEmpty;

impl LogTotalEmpty {
	#[inline(always)]
	pub const fn new() -> Self {
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
	fn lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteLock::empty_boxed()
	}

	fn lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteLock::empty_boxed()
	}

	fn no_flush_lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteNFLock::empty_boxed()
	}

	fn no_flush_lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteNFLock::empty_boxed()
	}
}


impl<'a> LogUnionConst<'a> for LogTotalEmpty {}
impl<'a> LogStatic<'a> for LogTotalEmpty {}
impl<'a> LogExtend<'a> for LogTotalEmpty {}

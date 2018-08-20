
use log_write::LogWrite;
use log_lock::LogSafeLock;
use log::LogLockIO;
use std::io::Stderr;
use std::io::Stdout;
use std::marker::PhantomData;
use std::io::StderrLock;
use std::io::StdoutLock;
use log::LogExtend;
use log::LogStatic;
use log::LogBase;
use log_addition::union::LogUnionConst;
use log::LogFlush;
use std::fmt::Arguments;
use std::io;
use std::io::Write;
use log_lock::default_nf::LogSafeWriteNFLock;
use log_lock::default::LogSafeWriteLock;


#[derive(Debug)]
pub struct LogEmpty<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>>(W, W2, PhantomData<&'a ()>);

impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogEmpty<'a, W, W2> {
	#[inline]
	pub fn new(w: W, w2: W2) -> Self {
		LogEmpty(w, w2, PhantomData)
	}
}

impl<'a> Default for LogEmpty<'a, Stdout, Stderr> {
	#[inline]
	fn default() -> LogEmpty<'a, Stdout, Stderr> {
		Self::new(io::stdout(), io::stderr())
	}
}


impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogBase<'a> for LogEmpty<'a, W, W2> {
	#[inline(always)]
	fn warning<'l>(&self, _args: Arguments<'l>) -> io::Result<()> {
		Ok( () )
	}

	#[inline(always)]
	fn trace<'l>(&self, _line: u32, _pos: u32, _file: &'static str, _args: Arguments<'l>) -> io::Result<()> {
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
	
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		self.0.lock().write_fmt(args)
	}
	
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		self.1.lock().write_fmt(args)
	}
}

impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogFlush for LogEmpty<'a, W, W2> {
	#[inline(always)]	
	fn flush_out(&self) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn flush_err(&self) -> io::Result<()> {
		Ok( () )
	}
}


impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogLockIO<'a> for LogEmpty<'a, W, W2> {
	fn lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteLock::boxed(self.0.lock())
	}

	fn lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteLock::boxed(self.1.lock())
	}

	fn no_flush_lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteNFLock::boxed(self.0.lock())
	}

	fn no_flush_lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteNFLock::boxed(self.0.lock())
	}
}

impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogUnionConst<'a> for LogEmpty<'a, W, W2> {}
impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogStatic<'a> for LogEmpty<'a, W, W2> {}
impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogExtend<'a> for LogEmpty<'a, W, W2> {}

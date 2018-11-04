
use log_core::LogLockIO;
use log_core::LogExtend;
use log_core::LogFlush;
use log_core::LogStatic;
use log_core::LogBase;
use log_write::LogWrite;
use std::io::Stderr;
use std::io::Stdout;
use std::marker::PhantomData;
use std::io::StderrLock;
use std::io::StdoutLock;
use std::fmt::Arguments;
use std::io;
use std::io::Write;

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
	fn no_flush_lock_out(&'a self) -> Box<Write + 'a> {
		//WriteGuard::boxed(self.0.lock())
		Box::new(self.0.lock())
	}

	fn no_flush_lock_err(&'a self) -> Box<Write + 'a> {
		//WriteGuard::boxed(self.0.lock())
		Box::new(self.0.lock())
	}
}

//impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogUnionConst<'a> for LogEmpty<'a, W, W2> {}
impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogStatic<'a> for LogEmpty<'a, W, W2> {}
impl<'a, W: LogWrite<'a, StdoutLock<'a>>, W2: LogWrite<'a, StderrLock<'a>>> LogExtend<'a> for LogEmpty<'a, W, W2> {}

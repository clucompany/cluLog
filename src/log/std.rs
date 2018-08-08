
use log::cluLogFlushIO;
use log::cluLogIOLock;
use std::ops::DerefMut;
use log::lock::cluLogLock;
use std::marker::PhantomData;
use ::write::LogWrite;
use ::panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use log::cluLog;
use ::std::io;

#[derive(Debug)]
pub struct LogStd<WRITER: LogWrite, PANIC: LogPanic> {
	_b:	PhantomData<WRITER>,
	_p:	PhantomData<PANIC>,	
	
	stdout: ::std::io::Stdout,
	stderr: ::std::io::Stderr,
}


impl<WRITER: LogWrite, PANIC: LogPanic> LogStd<WRITER, PANIC> {
	#[inline]
	pub fn new() -> Self {		
		Self {
			_b:	PhantomData,
			_p:	PhantomData,
			
			stdout: ::std::io::stdout(),
			stderr: ::std::io::stderr(),
		}
	}

	#[inline(always)]
	fn raw_lock_out<'a>(&'a self) -> impl Write + 'a {
		self.stdout.lock()
	}
	
	#[inline(always)]
	fn raw_lock_err<'a>(&'a self) -> impl Write + 'a {
		self.stderr.lock()
	}
}


impl<WRITER: LogWrite, PANIC: LogPanic> cluLog for LogStd<WRITER, PANIC> {
	#[inline]
	fn warning<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		WRITER::warning(&mut self.raw_lock_out(), args)
	}
	
	#[inline]
	fn info<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		WRITER::info(&mut self.raw_lock_out(), args)
	}
	
	#[inline]
	fn error<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		WRITER::error(&mut self.raw_lock_out(), args)
	}
	
	#[inline]
	fn panic<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		PANIC::panic::<WRITER>(&mut self.raw_lock_out(), args)
	}
	
	#[inline]
	fn unknown<'a>(&self, name: &'a str, args: Arguments<'a>) -> io::Result<()> {
		WRITER::unknown(&mut self.raw_lock_out(), name, args)
	}
	
	#[inline]
	fn print<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		WRITER::print(&mut self.raw_lock_out(), args)
	}
	
	#[inline]
	fn eprint<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		WRITER::eprint(&mut self.raw_lock_out(), args)
	}
}

impl<WRITER: LogWrite, PANIC: LogPanic> cluLogFlushIO for LogStd<WRITER, PANIC> {
	#[inline]
	fn flush_out(&self) -> io::Result<()> {
		self.raw_lock_out().flush()
	}
	
	#[inline]
	fn flush_err(&self) -> io::Result<()> {
		self.raw_lock_out().flush()
	}
}
	
impl<WRITER: LogWrite, PANIC: LogPanic> cluLogIOLock for LogStd<WRITER, PANIC> {
	#[inline]
	fn lock_out<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>> {
		cluLogLock::boxed(self.raw_lock_out())
	}
	
	#[inline]
	fn lock_err<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>> {
		cluLogLock::boxed(self.raw_lock_err())
	}

	#[inline]
	fn no_flush_lock_out<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>> {
		cluLogLock::boxed(self.raw_lock_out())
	}

	#[inline]
	fn no_flush_lock_err<'a>(&'a self) -> Box<'a + DerefMut<Target = Write + 'a>> {
		cluLogLock::boxed(self.raw_lock_err())
	}
}
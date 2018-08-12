
use std::ops::DerefMut;
use std::io::StderrLock;
use std::io::StdoutLock;
use log::raw_lock::cluLogRawIOLock;
use std::io::Stdout;
use std::io::Stderr;
use DefLogPanic;
use DefLogWrite;
use log::cluLogFlushIO;
use log::cluLogIOLock;
use log::lock::cluLogLock;
use log::lock::cluLogLockNoFlush;
use std::marker::PhantomData;
use ::write::LogWrite;
use ::panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use log::cluLog;
use ::std::io;

#[derive(Debug)]
pub struct LogStd<'a, WRITER: LogWrite, PANIC: LogPanic, O: cluLogRawIOLock<'a, OLOCK>, E: cluLogRawIOLock<'a, ELOCK>, OLOCK: 'a +  Write, ELOCK: 'a +  Write> {
	_b:	PhantomData<WRITER>,
	_p:	PhantomData<PANIC>,	
	_ln: PhantomData<&'a ()>,
	_pp: PhantomData<OLOCK>,
	_p2: PhantomData<ELOCK>,
	
	out: O,
	err: E,
}

impl<'a> Default for LogStd<'a, DefLogWrite, DefLogPanic, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	#[inline]
	fn default() -> Self {
		Self::new(io::stdout(), io::stderr())
	}
}

impl<'a> LogStd<'a, DefLogWrite, DefLogPanic, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	#[inline]
	pub fn default_box() -> Box<Self> {
		Box::new(Self::default())
	}
}



impl<'a, WRITER: LogWrite, PANIC: LogPanic, O: cluLogRawIOLock<'a, OLOCK>, E: cluLogRawIOLock<'a, ELOCK>, OLOCK: 'a +  Write, ELOCK: 'a +  Write> LogStd<'a, WRITER, PANIC, O, E, OLOCK, ELOCK> {
	#[inline]
	pub fn new(out: O, err: E) -> Self {		
		Self {
			_b:	PhantomData,
			_p:	PhantomData,
			_ln: PhantomData,
			_pp: PhantomData,
			_p2: PhantomData,
			
			out: out,
			err: err,
		}
	}
}


impl<'a, WRITER: LogWrite, PANIC: LogPanic, O: cluLogRawIOLock<'a, OLOCK>, E: cluLogRawIOLock<'a, ELOCK>, OLOCK: 'a + Write, ELOCK: 'a +  Write> cluLog<'a> for LogStd<'a, WRITER, PANIC, O, E, OLOCK, ELOCK> {
	fn warning<'l>(&'a self, args: Arguments) -> io::Result<()> {
		WRITER::warning(self.out.lock(), args)
	}
	
	fn info<'l>(&'a self, args: Arguments) -> io::Result<()> {
		WRITER::info(self.out.lock(), args)
	}
	
	fn error<'l>(&'a self, args: Arguments) -> io::Result<()> {
		WRITER::error(self.err.lock(), args)
	}
	
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		PANIC::panic::<WRITER, OLOCK>(self.out.lock(), args)
	}
	
	fn unknown<'l>(&'a self, name: &'l str, args: Arguments<'l>) -> io::Result<()> {
		WRITER::unknown(self.out.lock(), name, args)
	}
	
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		WRITER::print(self.out.lock(), args)
	}
	
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		WRITER::eprint(self.err.lock(), args)
	}
}

impl<'a, WRITER: LogWrite, PANIC: LogPanic, O: cluLogRawIOLock<'a, OLOCK>, E: cluLogRawIOLock<'a, ELOCK>, OLOCK: 'a + Write , ELOCK: 'a +  Write> cluLogFlushIO for LogStd<'a, WRITER, PANIC, O, E, OLOCK, ELOCK> {
	fn flush_out(&mut self) -> io::Result<()> {
		self.out.flush()
	}
	
	fn flush_err(&mut self) -> io::Result<()> {
		self.err.flush()
	}
}
	
impl<'a, WRITER: LogWrite, PANIC: LogPanic, O: cluLogRawIOLock<'a, OLOCK>, E: cluLogRawIOLock<'a, ELOCK>, OLOCK: 'a + Write , ELOCK: 'a +  Write> cluLogIOLock<'a> for LogStd<'a, WRITER, PANIC, O, E, OLOCK, ELOCK> {
	fn lock_out<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>> {
		cluLogLock::boxed(self.out.lock())
	}
	
	fn lock_err<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>> {
		cluLogLock::boxed(self.err.lock())
	}

	fn no_flush_lock_out<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>> {
		cluLogLockNoFlush::boxed(self.out.lock())
	}

	fn no_flush_lock_err<'l: 'a>(&'l self) -> Box<'l + DerefMut<Target = Write + 'l>> {
		cluLogLockNoFlush::boxed(self.err.lock())
	}
}

use log_addition::union::LogUnionConst;
use log_addition::empty::empty_write::EmptyWrite;
use log_addition::empty::LogEmptyConst;
use std::ops::DerefMut;
use std::io::StderrLock;
use std::io::StdoutLock;
use log::raw_lock::LogLockRawIO;
use std::io::Stdout;
use std::io::Stderr;
use DefLogPanic;
use DefLogWrite;
use log::LogFlushIO;
use log::LogLockIO;
use std::marker::PhantomData;
use log_write::LogWrite;
use log_panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use log::cluLog;
use std::io;
use log::lock::default_no_flush::LogLockNoFlush;
use log::lock::default::LogLock;

#[derive(Debug)]
pub struct LogStd<'a, W: LogWrite, P: LogPanic, O: LogLockRawIO<'a, OL>, E: LogLockRawIO<'a, EL>, OL: 'a +  Write, EL: 'a +  Write> {
	_b:	PhantomData<W>,
	_p:	PhantomData<P>,	
	_ln: PhantomData<&'a ()>,
	_pp: PhantomData<OL>,
	_p2: PhantomData<EL>,
	
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



impl<'a, W: LogWrite, P: LogPanic, O: LogLockRawIO<'a, OL>, E: LogLockRawIO<'a, EL>, OL: 'a +  Write, EL: 'a +  Write> LogStd<'a, W, P, O, E, OL, EL> {
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

impl<'a> LogEmptyConst for LogStd<'a, DefLogWrite, DefLogPanic, EmptyWrite, EmptyWrite, EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite, EmptyWrite)
	}
}
/*
impl<'a, W: LogWrite, P: LogPanic, O: LogLockRawIO<'a, OL>, E: LogLockRawIO<'a, EL>, OL: 'a +  Write, EL: 'a +  Write> LogUnionConst for LogStd<'a, W, P, O, E, OL, EL> {
	fn union<'a, B: cluLog<'a>>(self, b: B) -> LogUnion<'a, Self, B> {
		LogUnion::new(self, b)
	}
}*/


impl<'a, W: LogWrite, P: LogPanic, O: LogLockRawIO<'a, OL>, E: LogLockRawIO<'a, EL>, OL: 'a + Write, EL: 'a +  Write> cluLog<'a> for LogStd<'a, W, P, O, E, OL, EL> {
	fn warning<'l>(&'a self, args: Arguments) -> io::Result<()> {
		W::warning(self.out.lock(), args)
	}
	
	fn info<'l>(&'a self, args: Arguments) -> io::Result<()> {
		W::info(self.out.lock(), args)
	}
	
	fn error<'l>(&'a self, args: Arguments) -> io::Result<()> {
		W::error(self.err.lock(), args)
	}
	
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		P::panic::<W, OL>(self.out.lock(), args)
	}
	
	fn unknown<'l>(&'a self, name: &'static str, args: Arguments<'l>) -> io::Result<()> {
		W::unknown(self.out.lock(), name, args)
	}

	fn trace<'l>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'l>) -> io::Result<()> {
		W::trace(self.out.lock(), line, pos, file, args)
	}
	
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		W::print(self.out.lock(), args)
	}
	
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		W::eprint(self.err.lock(), args)
	}
}

impl<'a, W: LogWrite, P: LogPanic, O: LogLockRawIO<'a, OL>, E: LogLockRawIO<'a, EL>, OL: 'a + Write , EL: 'a +  Write> LogFlushIO for LogStd<'a, W, P, O, E, OL, EL> {
	fn flush_out(&mut self) -> io::Result<()> {
		self.out.flush()
	}
	
	fn flush_err(&mut self) -> io::Result<()> {
		self.err.flush()
	}
}
	
impl<'a, W: LogWrite, P: LogPanic, O: LogLockRawIO<'a, OL>, E: LogLockRawIO<'a, EL>, OL: 'a + Write , EL: 'a +  Write> LogLockIO<'a> for LogStd<'a, W, P, O, E, OL, EL> {
	fn lock_out(&'a self) -> Box<'a + Write> {
		LogLock::boxed(self.out.lock())
	}
	
	fn lock_err(&'a self) -> Box<'a + Write> {
		LogLock::boxed(self.err.lock())
	}

	fn no_flush_lock_out(&'a self) -> Box<'a + Write> {
		LogLockNoFlush::boxed(self.out.lock())
	}

	fn no_flush_lock_err(&'a self) -> Box<'a + Write> {
		LogLockNoFlush::boxed(self.err.lock())
	}
}



impl<'a, W: LogWrite, P: LogPanic, O: LogLockRawIO<'a, OL>, E: LogLockRawIO<'a, EL>, OL: 'a +  Write, EL: 'a +  Write> LogUnionConst<'a> for LogStd<'a, W, P, O, E, OL, EL> {}
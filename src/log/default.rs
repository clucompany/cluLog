
use log_lock::LogLock;
use log::LogLockIO;
use log_addition::union::LogUnionConst;
use log_addition::empty::empty_write::EmptyWrite;
use log_addition::empty::LogEmptyConst;
use std::io::StderrLock;
use std::io::StdoutLock;
use log::raw_lock::LogLockRawIO;
use std::io::Stdout;
use std::io::Stderr;
use DefLogPanic;
use DefLogWrite;
use log::LogFlush;
use std::marker::PhantomData;
use log_write::LogWrite;
use log_panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use log::LogExtend;
use log::LogStatic;
use log::LogBase;
use std::io;
use log_lock::default::LogSafeLock;
use log_lock::default_nf::LogSafeLockNF;


#[derive(Debug)]
pub struct LogDefault<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, E: LogLockRawIO<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> {
	_b:	PhantomData<W>,
	_p:	PhantomData<P>,	
	_ln: PhantomData<&'a ()>,
	_pp: PhantomData<OL>,
	_p2: PhantomData<EL>,
	
	out: O,
	err: E,
}

impl<'a> Default for LogDefault<'a, DefLogWrite, DefLogPanic, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	#[inline]
	fn default() -> Self {
		Self::new(io::stdout(), io::stderr())
	}
}

impl<'a> LogDefault<'a, DefLogWrite, DefLogPanic, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	/*#[inline]
	pub fn default_box() -> Box<Self> {
		Box::new(Self::default())
	}*/
}



impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, E: LogLockRawIO<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> LogDefault<'a, W, P, O, E, OL, EL> {
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

impl<'a> LogEmptyConst for LogDefault<'a, DefLogWrite, DefLogPanic, EmptyWrite, EmptyWrite, EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite, EmptyWrite)
	}
}

impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, E: LogLockRawIO<'a, EL> + Write, OL: 'a + Write, EL: 'a +  Write> LogBase<'a> for LogDefault<'a, W, P, O, E, OL, EL> {
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
		P::panic(self.out.lock(), args)
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

impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, E: LogLockRawIO<'a, EL> + Write, OL: 'a + Write , EL: 'a +  Write> LogFlush for LogDefault<'a, W, P, O, E, OL, EL> {
	fn flush_out(&mut self) -> io::Result<()> {
		self.out.flush()
	}
	
	fn flush_err(&mut self) -> io::Result<()> {
		self.err.flush()
	}
}
	
impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, E: LogLockRawIO<'a, EL> + Write, OL: 'a + Write , EL: 'a +  Write> LogLockIO<'a> for LogDefault<'a, W, P, O, E, OL, EL> {
	fn lock_out(&'a self) -> Box<LogLock<'a> + 'a> {
		LogSafeLock::boxed(self.out.lock())
	}
	
	fn lock_err(&'a self) -> Box<LogLock<'a> + 'a> {
		LogSafeLock::boxed(self.err.lock())
	}

	fn no_flush_lock_out(&'a self) -> Box<LogLock<'a> + 'a> {
		LogSafeLockNF::boxed(self.out.lock())
	}

	fn no_flush_lock_err(&'a self) -> Box<LogLock<'a> + 'a> {
		LogSafeLockNF::boxed(self.err.lock())
	}
}



impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, E: LogLockRawIO<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> LogUnionConst<'a> for LogDefault<'a, W, P, O, E, OL, EL> {}
impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, E: LogLockRawIO<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> LogStatic<'a> for LogDefault<'a, W, P, O, E, OL, EL> {}
impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, E: LogLockRawIO<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> LogExtend<'a> for LogDefault<'a, W, P, O, E, OL, EL> {}
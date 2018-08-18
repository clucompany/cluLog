
use log_lock::LogLock;
use log::LogLockIO;
use log_addition::union::LogUnionConst;
use log_addition::empty::empty_write::EmptyWrite;
use log_addition::empty::LogEmptyConst;
use std::io::StdoutLock;
use log::raw_lock::LogLockRawIO;
use std::io::Stdout;
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
pub struct LogOneDefault<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, OL: 'a +  Write> {
	_b:	PhantomData<W>,
	_p:	PhantomData<P>,	
	_ln: PhantomData<&'a ()>,
	_pp: PhantomData<OL>,
	
	out: O,
}

impl<'a> Default for LogOneDefault<'a, DefLogWrite, DefLogPanic, Stdout, StdoutLock<'a>> {
	#[inline]
	fn default() -> Self {
		Self::new(io::stdout())
	}
}

impl<'a> LogOneDefault<'a, DefLogWrite, DefLogPanic, Stdout, StdoutLock<'a>> {
	#[inline]
	pub fn default_box() -> Box<Self> {
		Box::new(Self::default())
	}
}



impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, OL: 'a +  Write> LogOneDefault<'a, W, P, O, OL> {
	#[inline]
	pub fn new(out: O) -> Self {		
		Self {
			_b:	PhantomData,
			_p:	PhantomData,
			_ln: PhantomData,
			_pp: PhantomData,
			
			out: out,
		}
	}
}

impl<'a> LogEmptyConst for LogOneDefault<'a, DefLogWrite, DefLogPanic, EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite)
	}
}

impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, OL: 'a + Write> LogBase<'a> for LogOneDefault<'a, W, P, O, OL> {
	fn warning<'l>(&'a self, args: Arguments) -> io::Result<()> {
		W::warning(self.out.lock(), args)
	}
	
	fn info<'l>(&'a self, args: Arguments) -> io::Result<()> {
		W::info(self.out.lock(), args)
	}
	
	fn error<'l>(&'a self, args: Arguments) -> io::Result<()> {
		W::error(self.out.lock(), args)
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
		W::eprint(self.out.lock(), args)
	}
}

impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, OL: 'a + Write> LogFlush for LogOneDefault<'a, W, P, O, OL> {
	fn flush_out(&mut self) -> io::Result<()> {
		self.out.flush()
	}
	
	fn flush_err(&mut self) -> io::Result<()> {
		self.out.flush()
	}
	#[inline]
	fn flush(&mut self) -> io::Result<()> {
		self.flush_out()
	}
}
	
impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, OL: 'a + Write> LogLockIO<'a> for LogOneDefault<'a, W, P, O, OL> {
	fn lock_out(&'a self) -> Box<LogLock<'a> + 'a> {
		LogSafeLock::boxed(self.out.lock())
	}
	
	fn lock_err(&'a self) -> Box<LogLock<'a> + 'a> {
		LogSafeLock::boxed(self.out.lock())
	}

	fn no_flush_lock_out(&'a self) -> Box<LogLock<'a> + 'a> {
		LogSafeLockNF::boxed(self.out.lock())
	}

	fn no_flush_lock_err(&'a self) -> Box<LogLock<'a> + 'a> {
		LogSafeLockNF::boxed(self.out.lock())
	}
}



impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, OL: 'a +  Write> LogUnionConst<'a> for LogOneDefault<'a, W, P, O, OL> {}
impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, OL: 'a +  Write> LogStatic<'a> for LogOneDefault<'a, W, P, O, OL> {}
impl<'a, W: LogWrite, P: LogPanic<W>, O: LogLockRawIO<'a, OL> + Write, OL: 'a +  Write> LogExtend<'a> for LogOneDefault<'a, W, P, O, OL> {}
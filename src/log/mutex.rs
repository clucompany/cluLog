/*
use std::sync::Mutex;
use log_lock::raw::LogWrite;
use log_lock::LogSafeLock;
use log::LogLockIO;
use log_addition::union::LogUnionConst;
use log_addition::empty::empty_write::EmptyWrite;
use log_addition::empty::LogEmptyConst;
use std::io::StderrLock;
use std::io::StdoutLock;
use std::io::Stdout;
use std::io::Stderr;
use DefLogPanic;
use DefLogWrite;
use log::LogFlush;
use std::marker::PhantomData;
use log_write::LogShape;
use log_panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use log::LogExtend;
use log::LogStatic;
use log::LogBase;
use std::io;
use log_lock::default::LogSafeWriteLock;
use log_lock::default_nf::LogSafeWriteNFLock;


#[derive(Debug)]
pub struct LogMutDefault<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, E: LogWrite<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> {
	_b:	PhantomData<W>,
	_p:	PhantomData<P>,	
	_ln: PhantomData<&'a ()>,
	_pp: PhantomData<OL>,
	_p2: PhantomData<EL>,
	
	out: Mutex<O>,
	err: Mutex<E>,
}

impl<'a> Default for LogMutDefault<'a, DefLogWrite, DefLogPanic, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	#[inline]
	fn default() -> Self {
		Self::new(io::stdout(), io::stderr())
	}
}

impl<'a> LogMutDefault<'a, DefLogWrite, DefLogPanic, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	/*#[inline]
	pub fn default_box() -> Box<Self> {
		Box::new(Self::default())
	}*/
}



impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, E: LogWrite<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> LogMutDefault<'a, W, P, O, E, OL, EL> {
	#[inline]
	pub fn new(out: O, err: E) -> Self {		
		Self {
			_b:	PhantomData,
			_p:	PhantomData,
			_ln: PhantomData,
			_pp: PhantomData,
			_p2: PhantomData,
			
			out: Mutex::new(out),
			err: Mutex::new(err),
		}
	}
     pub fn mutex(out: Mutex<O>, err: Mutex<E>) -> Self {
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

impl<'a> LogEmptyConst for LogMutDefault<'a, DefLogWrite, DefLogPanic, EmptyWrite, EmptyWrite, EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite, EmptyWrite)
	}
}

impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, E: LogWrite<'a, EL> + Write, OL: 'a + Write, EL: 'a +  Write> LogBase<'a> for LogMutDefault<'a, W, P, O, E, OL, EL> {
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

impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, E: LogWrite<'a, EL> + Write, OL: 'a + Write , EL: 'a +  Write> LogFlush for LogMutDefault<'a, W, P, O, E, OL, EL> {
	fn flush_out(&mut self) -> io::Result<()> {
		self.out.flush()
	}
	
	fn flush_err(&mut self) -> io::Result<()> {
		self.err.flush()
	}
}
	
impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, E: LogWrite<'a, EL> + Write, OL: 'a + Write , EL: 'a +  Write> LogLockIO<'a> for LogMutDefault<'a, W, P, O, E, OL, EL> {
	fn lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteLock::boxed(self.out.lock())
	}
	
	fn lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteLock::boxed(self.err.lock())
	}

	fn no_flush_lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteNFLock::boxed(self.out.lock())
	}

	fn no_flush_lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		LogSafeWriteNFLock::boxed(self.err.lock())
	}
}



impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, E: LogWrite<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> LogUnionConst<'a> for LogMutDefault<'a, W, P, O, E, OL, EL> {}
impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, E: LogWrite<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> LogStatic<'a> for LogMutDefault<'a, W, P, O, E, OL, EL> {}
impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, E: LogWrite<'a, EL> + Write, OL: 'a +  Write, EL: 'a +  Write> LogExtend<'a> for LogMutDefault<'a, W, P, O, E, OL, EL> {}


*/
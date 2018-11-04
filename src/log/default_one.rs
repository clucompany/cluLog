
use log_write::EmptyWrite;
use log_core::LogExtend;
use log_core::LogStatic;
use log_core::LogLockIO;
use log_core::LogBase;
use log_core::LogFlush;
use log_write::LogWrite;
use DefLogShape;
use log_addition::empty::LogEmptyConst;
use std::io::StdoutLock;
use std::io::Stdout;
use DefLogPanic;
use std::marker::PhantomData;
use log_shape::LogShape;
use log_panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use std::io;

#[allow(dead_code)]
pub type DefOneLog<'a> = LogOneDefault<'a, DefLogShape, DefLogPanic, Stdout, StdoutLock<'a>>;

///Log system with one out stream.
#[derive(Debug)]
pub struct LogOneDefault<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, OL: 'a +  Write> {
	_b:	PhantomData<W>,
	_p:	PhantomData<P>,	
	_ln: PhantomData<&'a ()>,
	_pp: PhantomData<OL>,
	
	out: O,
}

impl<'a> Default for LogOneDefault<'a, DefLogShape, DefLogPanic, Stdout, StdoutLock<'a>> {
	#[inline]
	fn default() -> Self {
		Self::new(io::stdout())
	}
}

impl<'a> LogOneDefault<'a, DefLogShape, DefLogPanic, Stdout, StdoutLock<'a>> {
	/*#[inline]
	pub fn default_box() -> Box<Self> {
		Box::new(Self::default())
	}*/
}



impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, OL: 'a +  Write> LogOneDefault<'a, W, P, O, OL> {
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

impl<'a> LogEmptyConst for LogOneDefault<'a, DefLogShape, DefLogPanic, EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite)
	}
}

impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, OL: 'a + Write> LogBase<'a> for LogOneDefault<'a, W, P, O, OL> {
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

impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, OL: 'a + Write> LogFlush for LogOneDefault<'a, W, P, O, OL> {
	fn flush_out(&self) -> io::Result<()> {
		self.out.un_flush()
	}
	
	fn flush_err(&self) -> io::Result<()> {
		self.out.un_flush()
	}
	#[inline]
	fn flush(&self) -> io::Result<()> {
		self.flush_out()
	}
}
	
impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, OL: 'a + Write> LogLockIO<'a> for LogOneDefault<'a, W, P, O, OL> {
	fn no_flush_lock_out(&'a self) -> Box<Write + 'a> {
		//WriteGuard::boxed(self.out.lock())
		Box::new(self.out.lock())
	}

	fn no_flush_lock_err(&'a self) -> Box<Write + 'a> {
		//WriteGuard::boxed(self.out.lock())
		Box::new(self.out.lock())
	}
}



//impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, OL: 'a +  Write> LogUnionConst<'a> for LogOneDefault<'a, W, P, O, OL> {}
impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, OL: 'a +  Write> LogStatic<'a> for LogOneDefault<'a, W, P, O, OL> {}
impl<'a, W: LogShape, P: LogPanic<W>, O: LogWrite<'a, OL> + Write, OL: 'a +  Write> LogExtend<'a> for LogOneDefault<'a, W, P, O, OL> {}
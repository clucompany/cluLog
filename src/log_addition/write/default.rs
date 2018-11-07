
use log_addition::LogEmptyConst;
use log_write::GuardEmptyWrite;
use log_write::EmptyWrite;
use log_core::LogLockIO;
use log_core::LogExtend;
use log_core::LogStatic;
use log_core::LogFlush;
use log_core::LogBase;
use log_core::LogShape;
use log_write::LogWrite;
use DefLogShape;
use std::io::StderrLock;
use std::io::StdoutLock;
use std::io::Stdout;
use std::io::Stderr;
use std::marker::PhantomData;
use std::fmt::Arguments;
use std::io::Write;
use std::io;


///Log system with two outgoing flows. Default logging system.
#[derive(Debug)]
pub struct LogDefault<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, E: LogWrite<'a, Lock = EL>, OL: 'a +  Write, EL: 'a +  Write> {
	_b:	PhantomData<W>,
	_ln: PhantomData<&'a ()>,
	_pp: PhantomData<OL>,
	_p2: PhantomData<EL>,
	
	out: O,
	err: E,
}

impl<'a> Default for LogDefault<'a, DefLogShape, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	#[inline]
	fn default() -> Self {
		Self::new(io::stdout(), io::stderr())
	}
}

impl<'a> LogDefault<'a, DefLogShape, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	/*#[inline]
	pub fn default_box() -> Box<Self> {
		Box::new(Self::default())
	}*/
}



impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, E: LogWrite<'a, Lock = EL>, OL: Write, EL: Write> LogDefault<'a, W, O, E, OL, EL> {
	#[inline]
	pub fn new(out: O, err: E) -> Self {		
		Self {
			_b:	PhantomData,
			_ln: PhantomData,
			_pp: PhantomData,
			_p2: PhantomData,
			
			out: out,
			err: err,
		}
	}
}

impl<'a> LogEmptyConst for LogDefault<'a, DefLogShape, EmptyWrite, EmptyWrite, GuardEmptyWrite, GuardEmptyWrite> {
	#[inline]
	fn empty() -> Self {
		LogDefault::new(EmptyWrite, EmptyWrite)
	}
}

impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, E: LogWrite<'a, Lock = EL>, OL: Write, EL: Write> LogBase<'a> for LogDefault<'a, W, O, E, OL, EL> {
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
		W::panic(self.err.lock(), args)
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

impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, E: LogWrite<'a, Lock = EL>, OL: Write , EL: Write> LogFlush<'a> for LogDefault<'a, W, O, E, OL, EL> {
	fn flush_out(&'a self) -> io::Result<()> {
		self.out.lock().flush()
	}
	
	fn flush_err(&'a self) -> io::Result<()> {
		self.err.lock().flush()
	}
}
	
impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, E: LogWrite<'a, Lock = EL>, OL: Write , EL: Write> LogLockIO<'a> for LogDefault<'a, W, O, E, OL, EL> {
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.out.lock())
		Box::new(self.out.lock())
	}

	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.err.lock())
		Box::new(self.err.lock())
	}
}



impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, E: LogWrite<'a, Lock = EL>, OL: Write, EL: Write> LogStatic<'a> for LogDefault<'a, W, O, E, OL, EL> {}
impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, E: LogWrite<'a, Lock = EL>, OL: Write, EL: Write> LogExtend<'a> for LogDefault<'a, W, O, E, OL, EL> {}



use log_addition::LogEmptyConst;
use log_write::GuardEmptyWrite;
use log_write::EmptyWrite;
use log_core::LogExtend;
use log_core::LogStatic;
use log_core::LogLockIO;
use log_core::LogBase;
use log_core::LogFlush;
use log_core::LogShape;
use log_write::LogWrite;
use DefLogShape;
use std::io::StdoutLock;
use std::io::Stdout;
use std::marker::PhantomData;
use std::fmt::Arguments;
use std::io::Write;
use std::io;

///Log system with one out stream.
#[derive(Debug)]
pub struct LogOneDefault<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, OL: 'a +  Write> {
	_w:	PhantomData<W>,
	_ln: PhantomData<&'a ()>,
	_ol: PhantomData<OL>,
	
	out: O,
}

impl<'a> Default for LogOneDefault<'a, DefLogShape, Stdout, StdoutLock<'a>> {
	#[inline]
	fn default() -> Self {
		Self::new(io::stdout())
	}
}



impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, OL: Write> LogOneDefault<'a, W, O, OL> {
	#[inline]
	pub fn new(out: O) -> Self {		
		Self {
			_w:	PhantomData,
			_ln: PhantomData,
			_ol: PhantomData,
			
			out: out,
		}
	}
}

impl<'a> LogEmptyConst for LogOneDefault<'a, DefLogShape, EmptyWrite, GuardEmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite)
	}
}

impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, OL: Write> LogBase<'a> for LogOneDefault<'a, W, O, OL> {
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
		W::panic(self.out.lock(), args)
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

impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, OL: Write> LogFlush<'a> for LogOneDefault<'a, W, O, OL> {
	fn flush_out(&'a self) -> io::Result<()> {
		self.out.lock().flush()
	}
	
	fn flush_err(&'a self) -> io::Result<()> {
		self.out.lock().flush()
	}
	#[inline]
	fn flush(&'a self) -> io::Result<()> {
		self.flush_out()
	}
}
	
impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, OL: Write> LogLockIO<'a> for LogOneDefault<'a, W, O, OL> {
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.out.lock())
		Box::new(self.out.lock())
	}

	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.out.lock())
		Box::new(self.out.lock())
	}
}


impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, OL: Write> LogStatic<'a> for LogOneDefault<'a, W, O, OL> {}
impl<'a, W: LogShape, O: LogWrite<'a, Lock = OL>, OL: Write> LogExtend<'a> for LogOneDefault<'a, W, O, OL> {}
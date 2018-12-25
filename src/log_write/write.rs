
use crate::log_core::LogExtend;
use crate::log_core::LogStatic;
use crate::log_core::LogLockIO;
use crate::log_core::LogFlush;
use crate::log_core::LogBase;
use crate::log_shape::DefLogShape;

use crate::log_core::LogShape;
use cluExtIO::ExtWrite;

use std::io::StdoutLock;
use std::io::Stdout;
use std::marker::PhantomData;
use std::fmt::Arguments;
use std::io::Write;
use std::io;

///Log system with one out stream.
#[derive(Debug)]
pub struct LogWrite<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: 'a +  Write> {
	_w:	PhantomData<W>,
	_ln: PhantomData<&'a ()>,
	_ol: PhantomData<OL>,
	
	out: O,
}

impl<'a> Default for LogWrite<'a, DefLogShape, Stdout, StdoutLock<'a>> {
	#[inline]
	fn default() -> Self {
		Self::new(io::stdout())
	}
}

impl<'a> LogWrite<'a, DefLogShape, Stdout, StdoutLock<'a>> {
	#[inline(always)]
	pub fn open_stdouterr() -> Self {
		Default::default()
	}
}



impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogWrite<'a, W, O, OL> {
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

impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogBase<'a> for LogWrite<'a, W, O, OL> {
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

impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogFlush<'a> for LogWrite<'a, W, O, OL> {
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
	
impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogLockIO<'a> for LogWrite<'a, W, O, OL> {
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.out.lock())
		Box::new(self.out.lock())
	}

	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.out.lock())
		Box::new(self.out.lock())
	}
}


impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogStatic<'a> for LogWrite<'a, W, O, OL> {}
impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogExtend<'a> for LogWrite<'a, W, O, OL> {}


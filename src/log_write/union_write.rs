
use crate::log_core::LogExtend;
use crate::log_core::LogStatic;
use crate::log_core::LogLockIO;
use crate::log_core::LogFlush;
use crate::log_core::LogBase;
use crate::log_shape::DefLogShape;

use cluExtIO::ExtWrite;
use crate::log_core::LogShape;

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
pub struct LogTwoWrite<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, E: ExtWrite<'a, LockWrite = EL>, OL: 'a +  Write, EL: 'a +  Write> {
	_b:	PhantomData<W>,
	_ln: PhantomData<&'a ()>,
	_pp: PhantomData<OL>,
	_p2: PhantomData<EL>,
	
	out: O,
	err: E,
}

impl<'a> Default for LogTwoWrite<'a, DefLogShape, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	#[inline]
	fn default() -> Self {
		Self::new(io::stdout(), io::stderr())
	}
}

impl<'a> LogTwoWrite<'a, DefLogShape, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>> {
	#[inline(always)]
	pub fn open_stdouterr() -> Self {
		Default::default()
	}
}

impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, E: ExtWrite<'a, LockWrite = EL>, OL: Write, EL: Write> LogTwoWrite<'a, W, O, E, OL, EL> {
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





impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, E: ExtWrite<'a, LockWrite = EL>, OL: Write, EL: Write> LogBase<'a> for LogTwoWrite<'a, W, O, E, OL, EL> {
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

impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, E: ExtWrite<'a, LockWrite = EL>, OL: Write , EL: Write> LogFlush<'a> for LogTwoWrite<'a, W, O, E, OL, EL> {
	fn flush_out(&'a self) -> io::Result<()> {
		self.out.lock().flush()
	}
	
	fn flush_err(&'a self) -> io::Result<()> {
		self.err.lock().flush()
	}
}
	
impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, E: ExtWrite<'a, LockWrite = EL>, OL: Write , EL: Write> LogLockIO<'a> for LogTwoWrite<'a, W, O, E, OL, EL> {
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.out.lock())
		Box::new(self.out.lock())
	}

	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.err.lock())
		Box::new(self.err.lock())
	}
}



impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, E: ExtWrite<'a, LockWrite = EL>, OL: Write, EL: Write> LogStatic<'a> for LogTwoWrite<'a, W, O, E, OL, EL> {}
impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, E: ExtWrite<'a, LockWrite = EL>, OL: Write, EL: Write> LogExtend<'a> for LogTwoWrite<'a, W, O, E, OL, EL> {}


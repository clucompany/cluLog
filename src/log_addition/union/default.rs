
//!Combining several log systems into one.

use log_core::LogStatic;
use log_core::LogLockIO;
use log_core::LogBase;
use log_core::LogFlush;
use log_core::LogExtend;
use log_lock::LogSafeLock;
use log_panic::LogPanic;
use std::fmt::Arguments;
use log_addition::empty::LogEmptyConst;
use log_addition::empty::total::LogTotalEmpty;
use std::marker::PhantomData;
use std::io;
use log_lock::UnionLock;

pub struct LogUnion<'a, A: 'a + LogExtend<'a> + Sized, B: 'a + LogExtend<'a> + Sized, Panic: LogPanic>(A, B, PhantomData<&'a ()>, PhantomData<Panic>);

impl<'a, A: 'a + LogExtend<'a>, B: 'a + LogExtend<'a>, Panic: LogPanic> LogUnion<'a, A, B, Panic> {
     #[inline]
     pub fn new(a: A, b: B) -> Self {
          LogUnion(
               a, b, PhantomData, PhantomData,
          )
     }
     #[inline]
     pub fn boxed(a: A, b: B) -> Box<Self> {
          Box::new(Self::new(a, b))
     }
     #[inline]
     pub fn to_box(self) -> Box<Self> {
          Box::new(self)
     }
}

impl<'a, A: 'a + LogExtend<'a> + Clone, B: 'a + LogExtend<'a> + Clone, Panic: LogPanic> Clone for LogUnion<'a, A, B, Panic> {
     fn clone(&self) -> Self {
          LogUnion::new(self.0.clone(), self.1.clone())
     }
}




impl<'a, Panic: LogPanic> LogEmptyConst for LogUnion<'a, LogTotalEmpty, LogTotalEmpty, Panic> {
     #[inline]
     fn empty() -> Self {
          Self::new(LogTotalEmpty, LogTotalEmpty)
     }
}



impl<'a, A: 'a + LogExtend<'a>, B: 'a + LogExtend<'a>, Panic: LogPanic> LogFlush for LogUnion<'a, A, B, Panic> {
     #[inline(always)]
     fn flush_out(&self) -> io::Result<()> {
          if let Err(e) = self.0.flush_out() {
               return Err(e);
          }
          self.1.flush_out()
     }

     #[inline(always)]
	fn flush_err(&self) -> io::Result<()> {
          if let Err(e) = self.0.flush_err() {
               return Err(e);
          }
          self.1.flush_err()
     }
}


impl<'a, A: LogExtend<'a>, B: LogExtend<'a>, Panic: LogPanic> LogBase<'a> for LogUnion<'a, A, B, Panic> {
     #[inline(always)]
	fn warning<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		if let Err(e) = self.0.warning(args) {
               return Err(e);
          }
          self.1.warning(args)
	}
	
     #[inline(always)]
	fn info<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		if let Err(e) = self.0.info(args) {
               return Err(e);
          }
          self.1.info(args)
	}
	
     #[inline(always)]
	fn error<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		if let Err(e) = self.0.error(args) {
               return Err(e);
          }
          self.1.error(args)
	}
	
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		Panic::panic(UnionLock::new(self.0.lock_err(), self.1.lock_err()), args)
	}
	
     #[inline(always)]
	fn unknown<'l>(&'a self, name: &'static str, args: Arguments<'l>) -> io::Result<()> {
		if let Err(e) = self.0.unknown(name, args) {
               return Err(e);
          }
          self.1.unknown(name, args)
	}

     #[inline(always)]
	fn trace<'l>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'l>) -> io::Result<()> {
		if let Err(e) = self.0.trace(line, pos, file, args) {
               return Err(e);
          }
          self.1.trace(line, pos, file, args)
	}
	
     #[inline(always)]
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		if let Err(e) = self.0.print(args) {
               return Err(e);
          }
          self.1.print(args)
	}
	
     #[inline(always)]
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		if let Err(e) = self.0.eprint(args) {
               return Err(e);
          }
          self.1.eprint(args)
	}
}



	
impl<'a, A: 'a + LogExtend<'a>, B: 'a + LogExtend<'a>, Panic: LogPanic> LogLockIO<'a> for LogUnion<'a, A, B, Panic> {
     #[inline(always)]
	fn no_flush_lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		UnionLock::boxed(self.0.lock_out(), self.1.lock_out())
	}

     #[inline(always)]
	fn no_flush_lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		UnionLock::boxed(self.0.lock_err(), self.1.lock_err())
	}
}






impl<'a, A: 'a + LogExtend<'a>, B: 'a + LogExtend<'a>, Panic: LogPanic> From< (A, B) > for LogUnion<'a, A, B, Panic> {
     #[inline]
     fn from((a, b): (A, B)) -> Self {
          Self::new(a, b)
     }
}


//impl<'a, A: LogExtend<'a>, B: LogExtend<'a>, Panic: LogPanic> LogUnionConst<'a> for LogUnion<'a, A, B, Panic> {}
impl<'a, A: LogExtend<'a>, B: LogExtend<'a>, Panic: LogPanic> LogExtend<'a> for LogUnion<'a, A, B, Panic> {}
impl<'a, A: LogExtend<'a>, B: LogExtend<'a>, Panic: LogPanic> LogStatic<'a> for LogUnion<'a, A, B, Panic> where Self: 'static {}

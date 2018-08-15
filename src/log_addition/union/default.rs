
use std::io::Stderr;
use log::cluLogStatic;
use log::Log;
use log_panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use log::lock::LogLockIO;
use log_addition::empty::LogEmptyConst;
use log_addition::empty::total::LogTotalEmpty;
use log_addition::empty::default::LogEmpty;
use log::LogFlushIO;
use std::marker::PhantomData;
use std::io;
use log_addition::union::lock::default::UnionLock;
use log_addition::union::lock::default_no_flush::UnionLockNoFlush;
use log_addition::union::LogUnionConst;
use log::cluLogExtend;
use std::io::Stdout;

pub struct LogUnion<'a, A: 'a + cluLogExtend<'a> + Sized, B: 'a + cluLogExtend<'a> + Sized, Panic: LogPanic>(A, B, PhantomData<&'a ()>, PhantomData<Panic>);

impl<'a, A: 'a + cluLogExtend<'a>, B: 'a + cluLogExtend<'a>, Panic: LogPanic> LogUnion<'a, A, B, Panic> {
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
}


impl<'a, Panic: LogPanic> LogEmptyConst for LogUnion<'a, LogTotalEmpty, LogTotalEmpty, Panic> {
     #[inline]
     fn empty() -> Self {
          Self::new(LogTotalEmpty, LogTotalEmpty)
     }
}



impl<'a, A: 'a + cluLogExtend<'a>, B: 'a + cluLogExtend<'a>, Panic: LogPanic> LogFlushIO for LogUnion<'a, A, B, Panic> {
     #[inline(always)]
     fn flush_out(&mut self) -> io::Result<()> {
          if let Err(e) = self.0.flush_out() {
               return Err(e);
          }
          if let Err(e) = self.1.flush_out() {
               return Err(e);
          }

          Ok( () )
     }

     #[inline(always)]
	fn flush_err(&mut self) -> io::Result<()> {
          if let Err(e) = self.0.flush_err() {
               return Err(e);
          }
          if let Err(e) = self.1.flush_err() {
               return Err(e);
          }

          Ok( () )
     }
}


impl<'a, A: 'a + cluLogExtend<'a>, B: 'a + cluLogExtend<'a>, Panic: LogPanic> Log<'a> for LogUnion<'a, A, B, Panic> {
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



	
impl<'a, A: 'a + cluLogExtend<'a>, B: 'a + cluLogExtend<'a>, Panic: LogPanic> LogLockIO<'a> for LogUnion<'a, A, B, Panic> {
     #[inline(always)]
	fn lock_out(&'a self) -> Box<'a + Write> {
		UnionLock::boxed(self.0.lock_out(), self.1.lock_out())
	}
	
     #[inline(always)]
	fn lock_err(&'a self) -> Box<'a + Write> {
		UnionLock::boxed(self.0.lock_err(), self.1.lock_err())
	}

     #[inline(always)]
	fn no_flush_lock_out(&'a self) -> Box<'a + Write> {
		UnionLockNoFlush::boxed(self.0.lock_out(), self.1.lock_out())
	}

     #[inline(always)]
	fn no_flush_lock_err(&'a self) -> Box<'a + Write> {
		UnionLockNoFlush::boxed(self.0.lock_err(), self.1.lock_err())
	}
}






impl<'a, A: 'a + cluLogExtend<'a>, B: 'a + cluLogExtend<'a>, Panic: LogPanic> From< (A, B) > for LogUnion<'a, A, B, Panic> {
     #[inline]
     fn from((a, b): (A, B)) -> Self {
          Self::new(a, b)
     }
}


impl<'a, A: 'a + cluLogExtend<'a>, B: 'a + cluLogExtend<'a>, Panic: LogPanic> LogUnionConst<'a> for LogUnion<'a, A, B, Panic> {}
impl<'a, A: 'a + cluLogExtend<'a>, B: 'a + cluLogExtend<'a>, Panic: LogPanic> cluLogExtend<'a> for LogUnion<'a, A, B, Panic> {}
impl<'a, A: 'a + cluLogExtend<'a>, B: 'a + cluLogExtend<'a>, Panic: LogPanic> cluLogStatic<'a> for LogUnion<'a, A, B, Panic> where Self: 'static {}

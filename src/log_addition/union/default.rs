
//!Combining several log systems into one.

use cluExtIO::UnionWrite;
use log_addition::LogEmptyConst;
use log_addition::LogTotalEmpty;
use std::io::Write;
use log_core::LogStatic;
use log_core::LogLockIO;
use log_core::LogBase;
use log_core::LogFlush;
use log_core::LogExtend;
use std::fmt::Arguments;
use std::marker::PhantomData;
use std::io;

pub struct LogUnion<'a, A: LogExtend<'a>, B: LogExtend<'a>>(A, B, PhantomData<&'a ()>);

impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogUnion<'a, A, B> {
     #[inline]
     pub fn new(a: A, b: B) -> Self {
          LogUnion(
               a, b, PhantomData,
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

impl<'a> LogEmptyConst for LogUnion<'a, LogTotalEmpty, LogTotalEmpty> {
	#[inline]
	fn empty() -> Self {
		LogUnion::new(LogTotalEmpty::new(), LogTotalEmpty::new())
	}
}



impl<'a, A: LogExtend<'a> + Clone, B: LogExtend<'a> + Clone> Clone for LogUnion<'a, A, B> {
     fn clone(&self) -> Self {
          LogUnion::new(self.0.clone(), self.1.clone())
     }
}


impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogFlush<'a> for LogUnion<'a, A, B> {
     #[inline(always)]
     fn flush_out(&'a self) -> io::Result<()> {
          let e = self.0.flush_out();
          let e2 = self.1.flush_out();
          if let Err(e) = e {
               return Err(e);
          }
          e2
     }

     #[inline(always)]
	fn flush_err(&'a self) -> io::Result<()> {
          let e = self.0.flush_err();
          let e2 = self.1.flush_err();
          if let Err(e) = e {
               return Err(e);
          }
          e2
     }
}


impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogBase<'a> for LogUnion<'a, A, B> {
     #[inline]
	fn warning<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.warning(args);
          let e2 = self.1.warning(args);
          if let Err(e) = e {
               return Err(e);
          }
          e2
	}
	
     #[inline]
	fn info<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.info(args);
          let e2 = self.1.info(args);
          if let Err(e) = e {
               return Err(e);
          }
          e2
	}
	
     #[inline]
	fn error<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.error(args);
          let e2 = self.1.error(args);
          if let Err(e) = e {
               return Err(e);
          }
          e2
	}
	
     #[inline]
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		//Panic::panic(UnionWrite::new(self.0.lock_err(), self.1.lock_err()), args)
          let e = self.0.panic(args);
          let e2 = self.1.panic(args);
          if let Err(e) = e {
               return Err(e);
          }
          e2
	}
	
     #[inline]
	fn unknown<'l>(&'a self, name: &'static str, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.unknown(name, args);
          let e2 = self.1.unknown(name, args);
          if let Err(e) = e {
               return Err(e);
          }
          e2
	}

     #[inline]
	fn trace<'l>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.trace(line, pos, file, args);
          let e2 = self.1.trace(line, pos, file, args);
          if let Err(e) = e {
               return Err(e);
          }
          e2
	}
	
     #[inline]
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
          let e = self.0.print(args);
          let e2 = self.1.print(args);
          if let Err(e) = e {
               return Err(e);
          }
          e2
	}
	
     #[inline]
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.eprint(args);
          let e2 = self.1.eprint(args);
          if let Err(e) = e {
               return Err(e);
          }
          e2
	}
}



	
impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogLockIO<'a> for LogUnion<'a, A, B> {
     #[inline]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		UnionWrite::boxed(self.0.raw_lock_out(), self.1.raw_lock_out())
	}

     #[inline]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		UnionWrite::boxed(self.0.raw_lock_out(), self.1.raw_lock_out())
	}
}


impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> From< (A, B) > for LogUnion<'a, A, B> {
     #[inline]
     fn from((a, b): (A, B)) -> Self {
          Self::new(a, b)
     }
}


impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogExtend<'a> for LogUnion<'a, A, B> {}
impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogStatic<'a> for LogUnion<'a, A, B> {}




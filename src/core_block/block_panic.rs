
use crate::core::LogStatic;
use crate::core::LogLockIO;
use crate::core::LogBase;
use crate::core::LogFlush;
use crate::core::LogExtend;

use std::io::Write;
use std::fmt::Arguments;
use std::marker::PhantomData;
use std::io;

#[derive(Debug)]
pub struct BlockLogPanic<'a, A: LogExtend<'a>>(A, PhantomData<&'a ()>);

impl<'a, A: LogExtend<'a>> BlockLogPanic<'a, A> {
     #[inline(always)]
     pub fn new(a: A) -> Self {
          BlockLogPanic(
               a, PhantomData,
          )
     }
     #[inline(always)]
     pub fn new_b(a: A) -> Box<Self> {
          Box::new(Self::new(a))
     }
	#[inline(always)]
     pub fn to_box(self) -> Box<Self> {
          Box::new(self)
     }
}

/*
impl<'a> LogEmptyConst for BlockLogPanic<'a, LogTotalEmpty, LogTotalEmpty> {
	#[inline(always)]
	fn empty() -> Self {
		BlockLogPanic::new(LogTotalEmpty::new(), LogTotalEmpty::new())
	}
}*/



impl<'a, A: LogExtend<'a> + Clone> Clone for BlockLogPanic<'a, A> {
     #[inline(always)]
     fn clone(&self) -> Self {
          BlockLogPanic::new(self.0.clone())
     }
}


impl<'a, A: LogExtend<'a>> LogFlush<'a> for BlockLogPanic<'a, A> {
     #[inline(always)]
     fn flush_out(&'a self) -> io::Result<()> {
          self.0.flush_out()
     }

     #[inline(always)]
	fn flush_err(&'a self) -> io::Result<()> {
          self.0.flush_err()
     }
}


impl<'a, A: LogExtend<'a>> LogBase<'a> for BlockLogPanic<'a, A> {
     #[inline(always)]
	fn warning<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		self.0.warning(args)
	}
	
     #[inline(always)]
	fn info<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		self.0.info(args)
	}
	
     #[inline(always)]
	fn error<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		self.0.error(args)
	}
	
     #[inline(always)]
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
          //self.0.panic(args)
          Ok( () )
	}
	
     #[inline(always)]
	fn unknown<'l>(&'a self, name: &'static str, args: Arguments<'l>) -> io::Result<()> {
		self.0.unknown(name, args)
	}

     #[inline(always)]
	fn trace<'l>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'l>) -> io::Result<()> {
		self.0.trace(line, pos, file, args)
	}
	
     #[inline(always)]
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
          self.0.print(args)
	}
	
     #[inline(always)]
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		self.0.eprint(args)
	}
}



	
impl<'a, A: LogExtend<'a>> LogLockIO<'a> for BlockLogPanic<'a, A> {
     #[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		self.0.raw_lock_err()
     }

     #[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		self.0.raw_lock_err()
	}
}


impl<'a, A: LogExtend<'a>> LogExtend<'a> for BlockLogPanic<'a, A> {}
impl<'a, A: LogExtend<'a>> LogStatic<'a> for BlockLogPanic<'a, A> {}


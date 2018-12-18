
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
pub struct BlockLogDebug<'a, A: LogExtend<'a>>(A, PhantomData<&'a ()>);

impl<'a, A: LogExtend<'a>> BlockLogDebug<'a, A> {
     #[inline(always)]
     pub fn new(a: A) -> Self {
          BlockLogDebug(
               a, PhantomData,
          )
     }
}

/*
impl<'a> LogEmptyConst for BlockLogDebug<'a, LogTotalEmpty, LogTotalEmpty> {
	#[inline(always)]
	fn empty() -> Self {
		BlockLogDebug::new(LogTotalEmpty::new(), LogTotalEmpty::new())
	}
}*/



impl<'a, A: LogExtend<'a> + Clone> Clone for BlockLogDebug<'a, A> {
     #[inline(always)]
     fn clone(&self) -> Self {
          BlockLogDebug::new(self.0.clone())
     }
}


impl<'a, A: LogExtend<'a>> LogFlush<'a> for BlockLogDebug<'a, A> {
     #[inline(always)]
     fn flush_out(&'a self) -> io::Result<()> {
          self.0.flush_out()
     }

     #[inline(always)]
	fn flush_err(&'a self) -> io::Result<()> {
          self.0.flush_err()
     }
}


impl<'a, A: LogExtend<'a>> LogBase<'a> for BlockLogDebug<'a, A> {
     #[inline(always)]
	fn warning<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		self.0.warning(args)
	}
	
     #[inline(always)]
	fn info<'l>(&'a self, _args: Arguments<'l>) -> io::Result<()> {
		//self.0.info(args)
          Ok( () )
	}
	
     #[inline(always)]
	fn error<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		self.0.error(args)
	}
	
     #[inline(always)]
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
          self.0.panic(args)
	}
	
     #[inline(always)]
	fn unknown<'l>(&'a self, _name: &'static str, _args: Arguments<'l>) -> io::Result<()> {
		//self.0.unknown(name, args)
          Ok( () )
	}

     #[inline(always)]
	fn trace<'l>(&'a self, _line: u32, _pos: u32, _file: &'static str, _args: Arguments<'l>) -> io::Result<()> {
		//self.0.trace(line, pos, file, args) BLOCK
          Ok( () )
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



	
impl<'a, A: LogExtend<'a>> LogLockIO<'a> for BlockLogDebug<'a, A> {
     #[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		self.0.raw_lock_err()
     }

     #[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		self.0.raw_lock_err()
	}
}


impl<'a, A: LogExtend<'a>> LogExtend<'a> for BlockLogDebug<'a, A> {}
impl<'a, A: LogExtend<'a>> LogStatic<'a> for BlockLogDebug<'a, A> {}


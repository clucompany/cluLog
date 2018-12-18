
use crate::core::LogStatic;
use crate::core::LogLockIO;
use crate::core::LogBase;
use crate::core::LogFlush;
use crate::core::LogExtend;
use cluExtIO::EmptyWrite;

use std::io::Write;
use std::fmt::Arguments;
use std::marker::PhantomData;
use std::io;

#[derive(Debug)]
pub struct BlockLogOut<'a, A: LogExtend<'a>>(A, PhantomData<&'a ()>);

impl<'a, A: LogExtend<'a>> BlockLogOut<'a, A> {
     #[inline(always)]
     pub fn new(a: A) -> Self {
          BlockLogOut(
               a, PhantomData,
          )
     }
}

/*
impl<'a> LogEmptyConst for BlockLogOut<'a, LogTotalEmpty, LogTotalEmpty> {
	#[inline(always)]
	fn empty() -> Self {
		BlockLogOut::new(LogTotalEmpty::new(), LogTotalEmpty::new())
	}
}*/



impl<'a, A: LogExtend<'a> + Clone> Clone for BlockLogOut<'a, A> {
     #[inline(always)]
     fn clone(&self) -> Self {
          BlockLogOut::new(self.0.clone())
     }
}


impl<'a, A: LogExtend<'a>> LogFlush<'a> for BlockLogOut<'a, A> {
     #[inline(always)]
     fn flush_out(&'a self) -> io::Result<()> {
          //self.0.flush_out()
          Ok( () )
     }

     #[inline(always)]
	fn flush_err(&'a self) -> io::Result<()> {
          self.0.flush_err()
     }
}


impl<'a, A: LogExtend<'a>> LogBase<'a> for BlockLogOut<'a, A> {
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
          self.0.panic(args)
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
	fn print<'l>(&'a self, _args: Arguments<'l>) -> io::Result<()> {
          Ok( () )
	}
	
     #[inline(always)]
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		self.0.eprint(args)
	}
}



	
impl<'a, A: LogExtend<'a>> LogLockIO<'a> for BlockLogOut<'a, A> {
     #[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		//self.0.raw_lock_err()
          EmptyWrite::new().into()
     }

     #[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		self.0.raw_lock_err()
	}
}


impl<'a, A: LogExtend<'a>> LogExtend<'a> for BlockLogOut<'a, A> {}
impl<'a, A: LogExtend<'a>> LogStatic<'a> for BlockLogOut<'a, A> {}


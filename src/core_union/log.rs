
//!Combining several log systems into one.

use cluExtIO::UnionWrite;
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
pub struct LogUnion<'a, A: LogExtend<'a>, B: LogExtend<'a>>(A, B, PhantomData<&'a ()>);

impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogUnion<'a, A, B> {
     #[inline(always)]
     pub fn new(a: A, b: B) -> Self {
          LogUnion(
               a, b, PhantomData,
          )
     }
     #[inline(always)]
     pub fn new_b(a: A, b: B) -> Box<Self>{
          Box::new(Self::new(a, b))
     }

     #[inline(always)]
     pub fn to_box(self) -> Box<Self> {
          Box::new(self)
     }
}

impl<'a, A: LogExtend<'a> + Clone, B: LogExtend<'a> + Clone> Clone for LogUnion<'a, A, B> {
     #[inline(always)]
     fn clone(&self) -> Self {
          LogUnion::new(self.0.clone(), self.1.clone())
     }
}


impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogFlush<'a> for LogUnion<'a, A, B> {
     #[inline(always)]
     fn flush_out(&'a self) -> io::Result<()> {
          let e = self.0.flush_out();
          let e2 = self.1.flush_out();
          if let Err(_) = e {
               return e;
          }

          e2
     }

     #[inline(always)]
	fn flush_err(&'a self) -> io::Result<()> {
          let e = self.0.flush_err();
          let e2 = self.1.flush_err();
          if let Err(_) = e {
               return e;
          }

          e2
     }
}


impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogBase<'a> for LogUnion<'a, A, B> {
     #[inline(always)]
	fn warning<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.warning(args);
          let e2 = self.1.warning(args);
          if let Err(_) = e {
               return e;
          }

          e2
	}
	
     #[inline(always)]
	fn info<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.info(args);
          let e2 = self.1.info(args);
          if let Err(e) = e {
               return Err(e);
          }

          e2
	}
	
     #[inline(always)]
	fn error<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.error(args);
          let e2 = self.1.error(args);
          if let Err(_) = e {
               return e;
          }

          e2
	}
	
     #[inline(always)]
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
          let e = self.0.panic(args);
          let e2 = self.1.panic(args);
          if let Err(_) = e {
               return e;
          }

          e2
	}
	
     #[inline(always)]
	fn unknown<'l>(&'a self, name: &'static str, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.unknown(name, args);
          let e2 = self.1.unknown(name, args);
          if let Err(_) = e {
               return e;
          }

          e2
	}

     #[inline(always)]
	fn trace<'l>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.trace(line, pos, file, args);
          let e2 = self.1.trace(line, pos, file, args);
          if let Err(_) = e {
               return e;
          }

          e2
	}
	
     #[inline(always)]
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
          let e = self.0.print(args);
          let e2 = self.1.print(args);
          if let Err(_) = e {
               return e;
          }

          e2
	}
	
     #[inline(always)]
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		let e = self.0.eprint(args);
          let e2 = self.1.eprint(args);
          if let Err(_) = e {
               return e;
          }

          e2
	}
}



	
impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogLockIO<'a> for LogUnion<'a, A, B> {
     #[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		Box::new(UnionWrite::new(self.0.raw_lock_out(), self.1.raw_lock_out()))
     }

     #[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		Box::new(UnionWrite::new(self.0.raw_lock_err(), self.1.raw_lock_err()))
	}
}


impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogExtend<'a> for LogUnion<'a, A, B> {}
impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> LogStatic<'a> for LogUnion<'a, A, B> {}






/*
Log_base![LogUnion< 'a + A = LogExtend<'a>, B = LogExtend<'a> >:

	trace[line, pos, file, args] => {
		let e = self.0.trace(line, pos, file, args);
          let e2 = self.1.trace(line, pos, file, args);
          if let Err(_) = e {
               return e;
          }

          e2
	};
	unknown[name, args] => {
		let e = self.0.unknown(name, args);
          let e2 = self.1.unknown(name, args);
          if let Err(_) = e {
               return e;
          }

          e2
	};

	warning[args] => {
		let e = self.0.warning(args);
          let e2 = self.1.warning(args);
          if let Err(_) = e {
               return e;
          }

          e2
	};
	info[args] => {
		let e = self.0.info(args);
          let e2 = self.1.info(args);
          if let Err(e) = e {
               return Err(e);
          }

          e2
	};
	error[args] => {
		let e = self.0.error(args);
          let e2 = self.1.error(args);
          if let Err(_) = e {
               return e;
          }

          e2
	};
	panic[args] => {
		let e = self.0.panic(args);
          let e2 = self.1.panic(args);
          if let Err(_) = e {
               return e;
          }

          e2
	};
	
	print[args] => {
		let e = self.0.print(args);
          let e2 = self.1.print(args);
          if let Err(_) = e {
               return e;
          }

          e2
	};
	eprint[args] => {
		let e = self.0.eprint(args);
          let e2 = self.1.eprint(args);
          if let Err(_) = e {
               return e;
          }

          e2
	};
];
*/


use log_addition::empty::LogEmptyConst;
use std::fmt::Debug;
use log_addition::empty::empty_write::EmptyWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::io;

///Blocking threads with automatic cleaning
#[allow(non_camel_case_types)]
pub struct LogLock<'a, W: Write + 'a>(W, PhantomData<&'a ()>);

impl<'a, W: Write + 'a> LogLock<'a, W> {
	#[inline]
	pub fn new(out: W) -> Self {
		LogLock(out, PhantomData)
	}
	#[inline]
	pub fn boxed(out: W) -> Box<Write + 'a> {
		Box::new(Self::new(out))
	}
}

impl<'a> LogEmptyConst for LogLock<'a, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite)
	}
}



impl<'a, W: Write + 'a> Debug for LogLock<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("LogLock { .. }")
	}
}

impl<'a, W: Write + 'a> Drop for LogLock<'a, W> {
	#[inline]
	fn drop(&mut self) {
		let _e = self.0.flush();
	}
}

impl<'a, W: Write + 'a> Write for LogLock<'a, W> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          self.0.write(buf)
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          self.0.flush()
     }
}

/*
impl<'a, W: Write + 'a, W2: Write + 'a> Add<LogLock<'a, W>> for LogLock<'a, W> {

}
*/

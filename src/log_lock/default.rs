
use log_lock::LogSafeLock;
use log_addition::empty::LogEmptyConst;
use std::fmt::Debug;
use log_addition::empty::empty_write::EmptyWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::io;

///Blocking threads with automatic cleaning
#[allow(non_camel_case_types)]
pub struct LogSafeWriteLock<'a, W: Write + 'a>(W, PhantomData<&'a ()>);

impl<'a, W: Write + 'a> LogSafeWriteLock<'a, W> {
	#[inline]
	pub fn new(out: W) -> Self {
		LogSafeWriteLock(out, PhantomData)
	}

	#[inline]
	pub fn impled(out: W) -> impl LogSafeLock<'a> + 'a {
		Self::new(out)
	}

	#[inline]
	pub fn boxed(out: W) -> Box<LogSafeLock<'a> + 'a>{
		Box::new(Self::new(out))
	}
}


impl<'a> LogEmptyConst for LogSafeWriteLock<'a, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite)
	}
}



impl<'a, W: Write + 'a> Debug for LogSafeWriteLock<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("LogSafeWriteLock { .. }")
	}
}

impl<'a, W: Write + 'a> Drop for LogSafeWriteLock<'a, W> {
	#[inline]
	fn drop(&mut self) {
		let _e = self.0.flush();
	}
}

impl<'a, W: Write + 'a> Write for LogSafeWriteLock<'a, W> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          self.0.write(buf)
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          self.0.flush()
     }
}


impl<'a, W: Write + 'a> LogSafeLock<'a> for LogSafeWriteLock<'a, W> {}


impl<'a, W: Write + 'a + Clone> Clone for LogSafeWriteLock<'a, W> {

	fn clone(&self) -> Self {
		Self::new(self.0.clone())
	}
}
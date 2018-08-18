
use log::lock::LogLock;
use log::lock::LogLockUnionConst;
use log_addition::empty::LogEmptyConst;
use std::fmt::Debug;
use log_addition::empty::empty_write::EmptyWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::io;

///Blocking threads with automatic cleaning
#[allow(non_camel_case_types)]
pub struct LogSafeLock<'a, W: Write + 'a>(W, PhantomData<&'a ()>);

impl<'a, W: Write + 'a> LogSafeLock<'a, W> {
	#[inline]
	pub fn new(out: W) -> Self {
		LogSafeLock(out, PhantomData)
	}

	#[inline]
	pub fn impled(out: W) -> impl LogLock<'a> + 'a {
		Self::new(out)
	}

	#[inline]
	pub fn boxed(out: W) -> Box<LogLock<'a> + 'a>{
		Box::new(Self::new(out))
	}
}


impl<'a> LogEmptyConst for LogSafeLock<'a, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite)
	}
}



impl<'a, W: Write + 'a> Debug for LogSafeLock<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("LogSafeLock { .. }")
	}
}

impl<'a, W: Write + 'a> Drop for LogSafeLock<'a, W> {
	#[inline]
	fn drop(&mut self) {
		let _e = self.0.flush();
	}
}

impl<'a, W: Write + 'a> Write for LogSafeLock<'a, W> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          self.0.write(buf)
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          self.0.flush()
     }
}


impl<'a, W: Write + 'a> LogLock<'a> for LogSafeLock<'a, W> {}
impl<'a, W: Write + 'a> LogLockUnionConst<'a> for LogSafeLock<'a, W> {}
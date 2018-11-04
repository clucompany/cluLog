
use log_write::EmptyWrite;
use log_addition::empty::LogEmptyConst;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::io::Write;

///Blocking threads with automatic cleaning
#[allow(non_camel_case_types)]
pub struct LogSafeWriteLock<'a, W: Write + 'a>(W, PhantomData<&'a ()>);

impl<'a, W: Write + 'a> LogSafeWriteLock<'a, W> {
	#[inline]
	pub fn new(out: W) -> Self {
		LogSafeWriteLock(out, PhantomData)
	}

	#[inline]
	pub fn boxed(out: W) -> Box<Write + 'a>{
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
/*
impl<'a, W: Write + 'a> Drop for LogSafeWriteLock<'a, W> {
	#[inline]
	fn drop(&mut self) {
		let _e = self.0.flush();
	}
}
*/

impl<'a, W: Write + 'a> Write for LogSafeWriteLock<'a, W> {
     #[inline(always)]
	fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
		self.0.write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> ::std::io::Result<()> {
		self.0.flush()
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> {
		self.0.write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: ::std::fmt::Arguments) -> ::std::io::Result<()> {
		self.0.write_fmt(fmt)
	}
}


//impl<'a, W: Write + 'a> LogSafeLock<'a> for LogSafeWriteLock<'a, W> {}


impl<'a, W: Write + 'a + Clone> Clone for LogSafeWriteLock<'a, W> {

	fn clone(&self) -> Self {
		Self::new(self.0.clone())
	}
}
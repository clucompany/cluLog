
use log_addition::empty::LogEmptyConst;
use log_addition::empty::empty_write::EmptyWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::fmt::Debug;
use std::io;

///Flow blocking without self-cleaning
#[allow(non_camel_case_types)]
pub struct LogLockNoFlush<'a, W: Write + 'a>(W, PhantomData<&'a ()>);

impl<'a, W: Write + 'a> LogLockNoFlush<'a, W> {
	#[inline]
	pub fn new(out: W) -> Self {
		LogLockNoFlush(out, PhantomData)
	}
	#[inline]
	pub fn boxed(out: W) -> Box<Write + 'a> {
		Box::new(Self::new(out))
	}
}


impl<'a> LogEmptyConst for LogLockNoFlush<'a, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		LogLockNoFlush::new(EmptyWrite)
	}
}

impl<'a, W: Write + 'a> Debug for LogLockNoFlush<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("LogLockNoFlush { .. }")
	}
}

impl<'a, W: Write + 'a> Write for LogLockNoFlush<'a, W> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          self.0.write(buf)
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          self.0.flush()
     }
}


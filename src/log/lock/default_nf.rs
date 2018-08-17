
use log_addition::empty::LogEmptyConst;
use log_addition::empty::empty_write::EmptyWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::fmt::Debug;
use std::io;

///Flow blocking without self-cleaning
#[allow(non_camel_case_types)]
pub struct LogSafeLockNF<'a, W: Write + 'a>(W, PhantomData<&'a ()>);

impl<'a, W: Write + 'a> LogSafeLockNF<'a, W> {
	#[inline]
	pub fn new(out: W) -> Self {
		LogSafeLockNF(out, PhantomData)
	}
	/*#[inline]
	pub fn boxed(out: W) -> Box<Write + 'a> {
		Box::new(Self::new(out))
	}*/
	#[inline]
	pub fn boxed(out: W) -> Box<Write + 'a> {
		Box::new(Self::new(out))
	}
}


impl<'a> LogEmptyConst for LogSafeLockNF<'a, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		LogSafeLockNF::new(EmptyWrite)
	}
}

impl<'a, W: Write + 'a> Debug for LogSafeLockNF<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("LogSafeLockNF { .. }")
	}
}

impl<'a, W: Write + 'a> Write for LogSafeLockNF<'a, W> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          self.0.write(buf)
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          self.0.flush()
     }
}

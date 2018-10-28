
use log_lock::LogSafeLock;
use log_addition::empty::LogEmptyConst;
use log_addition::empty::empty_write::EmptyWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::fmt::Debug;
use std::io;

///Flow blocking without self-cleaning
#[allow(non_camel_case_types)]
pub struct LogSafeWriteNFLock<'a, W: Write + 'a>(W, PhantomData<&'a ()>);

impl<'a, W: Write + 'a> LogSafeWriteNFLock<'a, W> {
	#[inline]
	pub fn new(out: W) -> Self {
		LogSafeWriteNFLock(out, PhantomData)
	}
	
	#[inline]
	pub fn boxed(out: W) -> Box<LogSafeLock<'a> + 'a> {
		Box::new(Self::new(out))
	}
}


impl<'a> LogEmptyConst for LogSafeWriteNFLock<'a, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		LogSafeWriteNFLock::new(EmptyWrite)
	}
}

impl<'a, W: Write + 'a> Debug for LogSafeWriteNFLock<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("LogSafeWriteNFLock { .. }")
	}
}

impl<'a, W: Write + 'a> Write for LogSafeWriteNFLock<'a, W> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          self.0.write(buf)
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          self.0.flush()
     }
}


impl<'a, W: Write + 'a> LogSafeLock<'a> for LogSafeWriteNFLock<'a, W> {}


impl<'a, W: Write + 'a + Clone> Clone for LogSafeWriteNFLock<'a, W> {

	fn clone(&self) -> Self {
		Self::new(self.0.clone())
	}
}
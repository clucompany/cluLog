
use std::sync::Mutex;
use std::sync::MutexGuard;
use log_lock::LogSafeLock;
use std::fmt::Debug;
use std::io::Write;

#[allow(non_camel_case_types)]
pub struct LogSafeMutexLock<'a, W: Write + 'a>(MutexGuard<'a, W>);

impl<'a, W: Write + 'a> LogSafeMutexLock<'a, W> {
	#[inline]
	pub fn new(out: MutexGuard<'a, W>) -> Self {
		LogSafeMutexLock(out)
	}

     pub fn mutex(mutex: &'a Mutex<W>) -> Self {
          let lock = match mutex.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          };
          Self::new(lock)
     }

	#[inline]
	pub fn impled(out: MutexGuard<'a, W>) -> impl LogSafeLock<'a> + 'a {
		Self::new(out)
	}

	#[inline]
	pub fn boxed(out: MutexGuard<'a, W>) -> Box<LogSafeLock<'a> + 'a>{
		Box::new(Self::new(out))
	}
}

impl<'a, W: Write + 'a> Debug for LogSafeMutexLock<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("LogSafeMutexLock { .. }")
	}
}

impl<'a, W: Write + 'a> Write for LogSafeMutexLock<'a, W> {
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

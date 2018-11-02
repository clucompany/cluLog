

use std::marker::PhantomData;
use std::io::Write;

mod default;
mod union;
mod mutex;

pub use self::default::*;
pub use self::union::*;
pub use self::mutex::*;



///The constructor of empty structures
pub trait LogSafeLock<'a>: Write + 'a {
     #[inline]
     fn union<B: LogSafeLock<'a> + Sized + 'a>(self, b: B) -> UnionLock<'a, Self, B> where Self: Sized { 
          UnionLock::new(self, b)
     }

	/*#[inline]
	fn union_nf<B: LogLockUnionConst<'a> + Sized + 'a>(self, b: B) -> UnionNFLock<'a, Self, B> where Self: Sized { 
          UnionNFLock::new(self, b)
     }*/
}

impl<'a, T: Write + 'a> LogSafeLock<'a> for T {}



#[derive(Debug)]
pub struct FlushOut<'a, T: LogSafeLock<'a> + 'a>(T, PhantomData<&'a ()>);

impl<'a, T: LogSafeLock<'a> + 'a> FlushOut<'a, T> {
	#[inline]
	pub fn new(a: T) -> Self {
		FlushOut(a, PhantomData)
	}

	#[inline(always)]
	pub fn flush(self) {}
}

impl<'a, T: LogSafeLock<'a> + 'a> Write for FlushOut<'a, T> {
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


impl<'a, T: LogSafeLock<'a> + 'a> Drop for FlushOut<'a, T> {
	#[inline(always)]
	fn drop(&mut self) {
		let _e = self.0.flush();
	}
}
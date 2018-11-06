
use std::marker::PhantomData;
use std::io::Write;


#[derive(Debug)]
pub struct FlushWrite<'a, T: Write + 'a>(T, PhantomData<&'a ()>);

impl<'a, T: Write + 'a> FlushWrite<'a, T> {
	#[inline]
	pub fn new(a: T) -> Self {
		FlushWrite(a, PhantomData)
	}

	#[inline(always)]
	pub fn flush(self) {}
}

impl<'a, T: Write + 'a> Write for FlushWrite<'a, T> {
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


impl<'a, T: Write + 'a> Drop for FlushWrite<'a, T> {
	#[inline(always)]
	fn drop(&mut self) {
		let _e = self.0.flush();
	}
}
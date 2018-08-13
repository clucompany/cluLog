
use std::io::Write;
use std::io;

#[derive(Debug, Default, Clone)]
pub struct EmptyWrite;

impl EmptyWrite {
	#[inline]
	pub fn new() -> Self {
		EmptyWrite
	}

	#[inline]
	pub fn impled() -> impl Write {
		Self::new()
	}

	#[inline]
	pub fn boxed() -> Box<Write> {
		Box::new(Self::new())
	}
}

impl Write for EmptyWrite {
	#[inline(always)]
	fn write<'a>(&mut self, _buf: &'a [u8]) -> io::Result<usize> {
		Ok( 0 )
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		Ok( () )
	}
}


